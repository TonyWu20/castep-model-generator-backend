// use cpt::{data::ELEMENT_TABLE, element::LookupElement};
use glob::glob;
use rayon::prelude::*;
use std::{
    error::Error,
    ffi::OsString,
    fs, io,
    path::{Path, PathBuf},
};

pub mod castep_param;

pub trait MyFilePath: AsRef<Path> + Into<OsString> + Clone {}

#[derive(Debug)]
pub struct ParamWriter<P: MyFilePath> {
    filestem: String,
    export_loc: P,
    potentials_loc: P,
    kptaux: KptAux,
    trjaux: TrjAux,
    task: String,
}

impl<P: MyFilePath> ParamWriter<P> {
    fn path_builder(&self, extension: &str) -> PathBuf {
        let dir_name = format!("{}_{}", self.filestem, self.task);
        let dir_loc = self.export_loc.clone().into();
        let export_loc = Path::new(&dir_loc);
        let filename = format!("{}.{}", self.filestem, extension);
        export_loc.join(dir_name).join(filename)
    }
    pub fn write_kptaux(&self) -> Result<(), io::Error> {
        todo!()
    }
}

#[derive(Debug)]
pub struct ParamWriterBuilder<P: MyFilePath> {
    filestem: String,
    export_loc: P,
    potentials_loc: Option<P>,
    kptaux: Option<KptAux>,
    trjaux: Option<TrjAux>,
    task: Option<String>,
}
impl<P: MyFilePath> ParamWriterBuilder<P> {
    pub fn new(filestem: &str, export_loc: P) -> Self {
        Self {
            filestem: filestem.to_string(),
            export_loc,
            potentials_loc: None,
            kptaux: None,
            trjaux: None,
            task: Some("opt".to_string()),
        }
    }
    pub fn with_potentials_loc(self, potentials_loc: P) -> Self {
        Self {
            filestem: self.filestem,
            export_loc: self.export_loc,
            potentials_loc: Some(potentials_loc.clone()),
            kptaux: self.kptaux,
            trjaux: self.trjaux,
            task: self.task,
        }
    }
    pub fn with_kptaux(self, kptaux: KptAux) -> Self {
        Self {
            filestem: self.filestem,
            export_loc: self.export_loc,
            potentials_loc: self.potentials_loc,
            kptaux: Some(kptaux),
            trjaux: self.trjaux,
            task: self.task,
        }
    }
    pub fn with_trjaux(self, trjaux: TrjAux) -> Self {
        Self {
            filestem: self.filestem,
            export_loc: self.export_loc,
            potentials_loc: self.potentials_loc,
            kptaux: self.kptaux,
            trjaux: Some(trjaux),
            task: self.task,
        }
    }
    pub fn set_task(self, task: &str) -> Self {
        Self {
            filestem: self.filestem,
            export_loc: self.export_loc,
            potentials_loc: self.potentials_loc,
            kptaux: self.kptaux,
            trjaux: self.trjaux,
            task: Some(task.into()),
        }
    }
    pub fn build(self) -> ParamWriter<P> {
        ParamWriter {
            filestem: self.filestem,
            export_loc: self.export_loc,
            potentials_loc: self.potentials_loc.unwrap(),
            kptaux: self.kptaux.unwrap(),
            trjaux: self.trjaux.unwrap(),
            task: self.task.unwrap(),
        }
    }
}
/// File `.kptaux`
#[derive(Debug)]
pub struct KptAux {
    kpoints: Vec<[f64; 4]>,
    /// This specifies the dimensions of the Monkhorst-Pack grid requested in the directions of the reciprocal space lattice vectors.
    mp_grid: [u8; 3],
    /**
    The single entry is the maximum distance between k-points on the Monkhorst-Pack grid. The dimensions of the grid will be chosen such that the maximum separation of k-points is less than this.
    Default unit is Å^-1
    */
    mp_spacing: Option<f64>,
    /// This specifies the offset of the Monkhorst-Pack grid with respect to the origin of the Brillouin zone. The three entries are the offset in fractional coordinates relative to the reciprocal lattice vectors.
    mp_offset: [f64; 3],
}

impl KptAux {
    pub fn export(&self) -> String {
        let [grid_x, grid_y, grid_z] = self.mp_grid;
        let mp_grid_text = format!("MP_GRID : {:>8}{:>8}{:>8}", grid_x, grid_y, grid_z);
        let [off_x, off_y, off_z] = self.mp_offset;
        let mp_offset_text = format!(
            "MP_OFFSET : {:22.18e}{:22.18e}{:22.18e}",
            off_x, off_y, off_z
        );
        format!(
            "{}\n{}\n{}",
            mp_grid_text,
            mp_offset_text,
            self.kpoint_images()
        )
    }
    /// Initial Rough version
    /// TODO: generate from `kpoints` and `mp_grid`
    fn kpoint_images(&self) -> String {
        r#"BLOCK KPOINT_IMAGES
   1   1
#ENDBLOCK KPOINT_IMAGES"#
            .into()
    }
}
/// File '.trjaux'
#[derive(Debug)]
pub struct TrjAux {
    /// Id ordered as in `cell`. Directly imported from `LatticeModel<CellModel>`
    atom_ids: Vec<u32>,
}

impl TrjAux {
    pub fn export(&self) -> String {
        {
            let mut trjaux_contents = String::new();
            let trjaux_header = r#"# Atom IDs to appear in any .trj file to be generated.
# Correspond to atom IDs which will be used in exported .msi file
# required for animation/analysis of trajectory within Cerius2.
"#;
            trjaux_contents.push_str(trjaux_header);
            self.atom_ids.iter().for_each(|atom_id| {
                trjaux_contents.push_str(&format!("{}\n", atom_id));
            });
            let trjaux_ending = r#"#Origin  0.000000000000000e+000  0.000000000000000e+000  0.000000000000000e+000"#;
            trjaux_contents.push_str(trjaux_ending);
            trjaux_contents
        }
    }
}
pub fn to_xsd_scripts(target_root_dir: &str) -> Result<(), Box<dyn Error>> {
    let msi_pattern = format!("{target_root_dir}/**/*.msi");
    let item_collection = glob(&msi_pattern)
        .expect("Failed to read glob pattern")
        .into_iter()
        .par_bridge()
        .into_par_iter()
        .map(|entry| -> Option<String> {
            match entry {
                Ok(path) => {
                    let stem = path.file_stem().unwrap();
                    let parent = path.parent().unwrap();
                    Some(format!(
                        r#""{}/{}""#,
                        parent.to_str().unwrap(),
                        stem.to_str().unwrap()
                    ))
                }
                Err(e) => {
                    println!("glob entry match error: {:?}", e);
                    None
                }
            }
        })
        .collect::<Vec<Option<String>>>()
        .iter()
        .map(|entry| -> String { entry.as_ref().unwrap().to_string() })
        .collect::<Vec<String>>();
    let all_files_text = item_collection.join(", ");
    let headlines = r#"#!perl
use strict;
use Getopt::Long;
use MaterialsScript qw(:all);
"#;
    let array_text = format!("my @params = (\n{});\n", all_files_text);
    let actions = r#"foreach my $item (@params) {
    my $doc = $Documents{"${item}.msi"};
    $doc->CalculateBonds;
    $doc->Export("${item}.xsd");
    $doc->Save;
    $doc->Close;
}"#;
    let contents = format!("{headlines}{array_text}{actions}");
    fs::write(Path::new("msi_to_xsd.pl"), contents)?;
    Ok(())
}

// pub trait ParamWriter {
//     fn write_seed_files(
//         &mut self,
//         target_root_dir: &str,
//         potentials_loc: &str,
//     ) -> Result<(), Box<dyn Error>> {
//         let export_dir = self.export_destination(target_root_dir)?;
//         let msi_path = export_dir.join(&format!("{}.msi", self.get_lattice_name()));
//         if !msi_path.exists() {
//             let msi_content = self.output_in_msi();
//             fs::write(msi_path, msi_content)?;
//         } else {
//             let moved_dest = export_dir.join(&msi_path.file_name().unwrap());
//             if moved_dest.exists() == false {
//                 fs::rename(&msi_path, moved_dest)?;
//             }
//         }
//         self.rotate_to_standard_orientation();
//         let cell_path = self.export_filepath(target_root_dir, ".cell")?;
//         fs::write(cell_path, self.cell_output())?;
//         self.write_param(target_root_dir, potentials_loc)?;
//         self.write_kptaux(target_root_dir)?;
//         self.write_trjaux(target_root_dir)?;
//         #[cfg(not(debug_assertions))]
//         self.copy_potentials(target_root_dir, potentials_loc)?;
//         self.copy_smcastep_extension(target_root_dir)?;
//         self.write_lsf_script(target_root_dir)?;
//         Ok(())
//     }
//     fn export_destination(&self, target_root_dir: &str) -> Result<PathBuf, Box<dyn Error>> {
//         let lattice_name = self.get_lattice_name();
//         let dir_path = format!("{target_root_dir}/{lattice_name}_opt");
//         create_dir_all(&dir_path)?;
//         Ok(Path::new(&dir_path).to_path_buf())
//     }
//     fn export_filepath(
//         &self,
//         target_root_dir: &str,
//         file_extension: &str,
//     ) -> Result<PathBuf, Box<dyn Error>> {
//         let export_dest = self.export_destination(target_root_dir)?;
//         let filename = format!("{}{}", self.get_lattice_name(), file_extension);
//         Ok(export_dest.join(filename))
//     }
//     fn get_final_cutoff_energy(&self, potentials_loc: &str) -> f64 {
//         let mut energy: f64 = 0.0;
//         self.list_element().iter().for_each(|elm| {
//             let potential_file = ELEMENT_TABLE.get_by_symbol(elm).unwrap().potential();
//             let potential_path = format!("{potentials_loc}/{potential_file}");
//             let file = File::open(&potential_path).unwrap();
//             let file = BufReader::new(file);
//             let fine_energy: u32 = file
//                 .lines()
//                 .find(|line| line.as_ref().unwrap().contains("FINE"))
//                 .map(|line| {
//                     let num_str = line.as_ref().unwrap().split_whitespace().next().unwrap();
//                     num_str.parse::<u32>().unwrap()
//                 })
//                 .unwrap();
//             let round_bigger_tenth = |num: u32| -> f64 {
//                 match num % 10 {
//                     0 => num as f64,
//                     _ => ((num / 10 + 1) * 10) as f64,
//                 }
//             };
//             let ultra_fine_energy = round_bigger_tenth((fine_energy as f64 * 1.1) as u32);
//             energy = if energy > ultra_fine_energy {
//                 energy
//             } else {
//                 ultra_fine_energy
//             };
//         });
//         energy
//     }
//     fn write_param(
//         &self,
//         target_root_dir: &str,
//         potentials_loc: &str,
//     ) -> Result<(), Box<dyn Error>> {
//         let geom_param_path = self.export_filepath(target_root_dir, ".param")?;
//         if !geom_param_path.exists() {
//             let cutoff_energy = self.get_final_cutoff_energy(potentials_loc);
//             let spin_total = self
//                 .get_atoms()
//                 .iter()
//                 .map(|atom| -> u8 {
//                     ELEMENT_TABLE
//                         .get_by_symbol(atom.element_symbol())
//                         .unwrap()
//                         .spin()
//                 })
//                 .reduce(|total, i| total + i)
//                 .unwrap();
//             let geom_param_content = format!(
//                 r#"task : GeometryOptimization
// comment : CASTEP calculation from Materials Studio
// xc_functional : PBE
// spin_polarized : true
// spin :        {spin_total}
// opt_strategy : Speed
// page_wvfns :        0
// cut_off_energy :      {cutoff_energy:18.15}
// grid_scale :        1.500000000000000
// fine_grid_scale :        1.500000000000000
// finite_basis_corr :        0
// elec_energy_tol :   1.000000000000000e-005
// max_scf_cycles :     6000
// fix_occupancy : false
// metals_method : dm
// mixing_scheme : Pulay
// mix_charge_amp :        0.500000000000000
// mix_spin_amp :        2.000000000000000
// mix_charge_gmax :        1.500000000000000
// mix_spin_gmax :        1.500000000000000
// mix_history_length :       20
// perc_extra_bands : 72
// smearing_width :        0.100000000000000
// spin_fix :        6
// num_dump_cycles : 0
// geom_energy_tol :   5.000000000000000e-005
// geom_force_tol :        0.100000000000000
// geom_stress_tol :        0.200000000000000
// geom_disp_tol :        0.005000000000000
// geom_max_iter :     6000
// geom_method : BFGS
// fixed_npw : false
// calculate_ELF : false
// calculate_stress : false
// popn_calculate : true
// calculate_hirshfeld : true
// calculate_densdiff : false
// popn_bond_cutoff :        3.000000000000000
// pdos_calculate_weights : true
// "#
//             );
//             fs::write(&geom_param_path, geom_param_content)?;
//         }
//         let dos_param_path = self.export_filepath(target_root_dir, "_DOS.param")?;
//         if !dos_param_path.exists() {
//             let cutoff_energy = self.get_final_cutoff_energy(potentials_loc);
//             let spin_total = self
//                 .get_atoms()
//                 .iter()
//                 .map(|atom| -> u8 {
//                     ELEMENT_TABLE
//                         .get_by_symbol(atom.element_symbol())
//                         .unwrap()
//                         .spin
//                 })
//                 .reduce(|total, i| total + i)
//                 .unwrap();
//             let dos_param_content = format!(
//                 r#"task : BandStructure
// continuation : default
// comment : CASTEP calculation from Materials Studio
// xc_functional : PBE
// spin_polarized : true
// spin :        {spin_total}
// opt_strategy : Speed
// page_wvfns :        0
// cut_off_energy :      {cutoff_energy:.15}
// grid_scale :        1.500000000000000
// fine_grid_scale :        1.500000000000000
// finite_basis_corr :        0
// elec_energy_tol :   1.000000000000000e-005
// max_scf_cycles :     6000
// fix_occupancy : false
// metals_method : dm
// mixing_scheme : Pulay
// mix_charge_amp :        0.500000000000000
// mix_spin_amp :        2.000000000000000
// mix_charge_gmax :        1.500000000000000
// mix_spin_gmax :        1.500000000000000
// mix_history_length :       20
// perc_extra_bands :      72
// smearing_width :        0.100000000000000
// spin_fix :        6
// num_dump_cycles : 0
// bs_nextra_bands :       72
// bs_xc_functional : PBE
// bs_eigenvalue_tol :   1.000000000000000e-005
// calculate_stress : false
// calculate_ELF : false
// popn_calculate : false
// calculate_hirshfeld : false
// calculate_densdiff : false
// pdos_calculate_weights : true
// bs_write_eigenvalues : true
// "#
//             );
//             fs::write(dos_param_path, dos_param_content)?;
//         }
//         Ok(())
//     }
//     fn write_kptaux(&self, target_root_dir: &str) -> Result<(), Box<dyn Error>> {
//         let kptaux_contents = r#"MP_GRID :        1       1       1
// MP_OFFSET :   0.000000000000000e+000
// 0.000000000000000e+000  0.000000000000000e+000
// %BLOCK KPOINT_IMAGES
//    1   1
// %ENDBLOCK KPOINT_IMAGES"#
//             .to_string();
//         let kptaux_path = self.export_filepath(target_root_dir, ".kptaux")?;
//         if !kptaux_path.exists() {
//             fs::write(kptaux_path, &kptaux_contents).unwrap_or_else(|_| {
//                 panic!("Unable to write kptaux for {}", self.get_lattice_name())
//             });
//         }
//         let kptaux_dos_path = self.export_filepath(target_root_dir, "_DOS.kptaux")?;
//         if !kptaux_dos_path.exists() {
//             fs::write(kptaux_dos_path, &kptaux_contents).unwrap_or_else(|_| {
//                 panic!("Unable to write dos_kptaux for {}", self.get_lattice_name())
//             });
//         }
//         Ok(())
//     }
//     fn write_trjaux(&self, target_root_dir: &str) -> Result<(), Box<dyn Error>> {
//         let trjaux_path = self.export_filepath(target_root_dir, ".trjaux")?;
//         if !trjaux_path.exists() {
//             let mut trjaux_contents = String::new();
//             let trjaux_header = r#"# Atom IDs to appear in any .trj file to be generated.
// # Correspond to atom IDs which will be used in exported .msi file
// # required for animation/analysis of trajectory within Cerius2.
// "#;
//             trjaux_contents.push_str(trjaux_header);
//             self.get_atoms().iter().for_each(|atom| {
//                 trjaux_contents.push_str(&format!("{}\n", atom.atom_id()));
//             });
//             let trjaux_ending = r#"#Origin  0.000000000000000e+000  0.000000000000000e+000  0.000000000000000e+000"#;
//             trjaux_contents.push_str(trjaux_ending);
//             fs::write(trjaux_path, trjaux_contents)?;
//         }
//         Ok(())
//     }
//     fn copy_potentials(
//         &self,
//         target_root_dir: &str,
//         potentials_loc: &str,
//     ) -> Result<(), Box<dyn Error>> {
//         let target_dir = self.export_destination(target_root_dir)?;
//         self.list_element()
//             .iter()
//             .try_for_each(|elm| -> Result<(), Box<dyn Error>> {
//                 let pot_file = ELEMENT_TABLE.get_by_symbol(elm).unwrap().potential();
//                 let original_file = format!("{potentials_loc}/{pot_file}");
//                 let original_path = Path::new(&original_file);
//                 let dest_path = target_dir.join(pot_file);
//                 if !dest_path.exists() {
//                     fs::copy(original_path, dest_path)?;
//                     Ok(())
//                 } else {
//                     Ok(())
//                 }
//             })?;
//         Ok(())
//     }
//     fn copy_smcastep_extension(&self, target_root_dir: &str) -> Result<(), Box<dyn Error>>;
//     fn write_lsf_script(&self, target_root_dir: &str) -> Result<(), Box<dyn Error>> {
//         let target_dir = self.export_destination(target_root_dir)?;
//         let cell_name = self.get_lattice_name();
//         let cmd = format!("/home-yw/Soft/msi/MS70/MaterialsStudio7.0/etc/CASTEP/bin/RunCASTEP.sh -np $NP {cell_name}");
//         let prefix = r#"APP_NAME=intelY_mid
// NP=12
// NP_PER_NODE=12
// OMP_NUM_THREADS=1
// RUN="RAW"
//
// "#;
//         let content = format!("{prefix}{cmd}");
//         let lsf_filepath = target_dir.join("MS70_YW_CASTEP.lsf");
//         fs::write(lsf_filepath, content)?;
//         Ok(())
//     }
// }
