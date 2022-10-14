pub mod param_writer {
    use indicatif::ProgressBar;
    use std::error::Error;
    use std::fs::{self, create_dir_all};
    use std::path::{Path, PathBuf};
    use std::{collections::HashMap, fs::read_to_string};

    use crate::atom::AtomArrayRef;
    use crate::external_info::element_table::{self, Element};
    use crate::external_info::YamlTable;
    use crate::lattice::Lattice;
    use crate::parser::msi_parser::parse_lattice;
    use crate::Export;
    use crate::{atom::Atom, cell::CellOutput};
    use glob::glob;
    use rayon::prelude::*;
    use regex::Regex;

    pub fn generate_all_seed_files<P: AsRef<Path>>(
        element_tab_loc: P,
        target_root_dir: &str,
        potentials_loc: &str,
    ) -> Result<(), Box<dyn Error>> {
        let element_tab = element_table::ElmTab::load_table(element_tab_loc)?;
        let element_infotab = element_tab.hash_table()?;
        let msi_pattern = format!("{target_root_dir}/**/*.msi");
        let file_iter = glob(&msi_pattern)
            .expect("Failed to read glob pattern")
            .into_iter();
        let file_count = glob(&msi_pattern)
            .expect("Failed to read glob pattern")
            .into_iter()
            .count();
        let bar = ProgressBar::new(file_count as u64);
        file_iter
            .par_bridge()
            .into_par_iter()
            .for_each(|entry| match entry {
                Ok(path) => {
                    let mut lattice =
                        parse_lattice(path.to_str().unwrap()).unwrap_or_else(|e| panic!("{}", e));
                    lattice.sort_atoms_by_elements();
                    write_seed_files_for_cell(
                        &mut lattice,
                        target_root_dir,
                        &element_infotab,
                        potentials_loc,
                    )
                    .expect("Write seed files fail");
                    bar.inc(1);
                }
                Err(e) => println!("glob entry match error: {:?}", e),
            });
        Ok(bar.finish())
    }
    pub fn to_xsd_scripts(target_root_dir: &str) {
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
        fs::write(Path::new("msi_to_xsd.pl"), contents).expect("Failed writing msi_to_xsd.pl");
    }

    pub fn write_seed_files_for_cell(
        lattice: &mut Lattice,
        target_root_dir: &str,
        element_infotab: &HashMap<String, Element>,
        potentials_loc: &str,
    ) -> Result<(), Box<dyn Error>> {
        let cell_output = lattice.cell_output(element_infotab);
        let cell_path = export_filepath(lattice, target_root_dir, ".cell")?;
        fs::write(cell_path, cell_output)?;
        write_param(lattice, target_root_dir, element_infotab, potentials_loc)?;
        write_kptaux(lattice, target_root_dir)?;
        write_trjaux(lattice, target_root_dir)?;
        #[cfg(not(debug_assertions))]
        copy_potentials(lattice, target_root_dir, element_infotab, potentials_loc)?;
        copy_smcastep_extension(lattice, target_root_dir)?;
        // Currently asked for lsf
        write_lsf_script(lattice, target_root_dir)?;
        let export_dir = export_destination(lattice, target_root_dir)?;
        let msi_path = export_dir
            .parent()
            .unwrap()
            .join(&format!("{}.msi", lattice.lattice_name()));
        if msi_path.exists() == false {
            let msi_content = lattice.format_output();
            fs::write(msi_path, msi_content)?;
        } else {
            let moved_dest = export_dir.join(&msi_path.file_name().unwrap());
            if moved_dest.exists() == false {
                fs::rename(&msi_path, moved_dest)?;
            }
        }
        Ok(())
    }

    pub fn export_destination(lattice: &Lattice, target_root_dir: &str) -> Result<PathBuf, String> {
        let main_metal_element: &Atom = &lattice
            .atoms_vec()
            .get_atom_by_id(lattice.get_metal_sites()[0 as usize])?
            .to_owned();
        let metal_name = main_metal_element.element_name();
        let metal_id = main_metal_element.element_id();
        let family = match metal_id {
            21..=30 => "3d",
            39..=48 => "4d",
            72..=80 => "5d",
            57..=71 => "rare_earth",
            _ => "else",
        };
        let dir_path = format!(
            "{target_root_dir}/{}/{}/{}_opt",
            family,
            metal_name,
            lattice.lattice_name()
        );
        create_dir_all(&dir_path).unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
        Ok(Path::new(&dir_path).to_path_buf())
    }

    fn export_filepath(
        lattice: &Lattice,
        target_root_dir: &str,
        filename: &str,
    ) -> Result<PathBuf, String> {
        let export_dest = export_destination(lattice, target_root_dir)?;
        let export_filename = format!("{}{}", lattice.lattice_name(), filename);
        Ok(export_dest.join(export_filename))
    }

    fn get_final_cutoff_energy(
        lattice: &Lattice,
        element_infotab: &HashMap<String, Element>,
        potentials_loc: &str,
    ) -> f64 {
        let mut energy: f64 = 0.0;
        let element_lists = lattice.get_element_list();
        let fine_cutoff_energy_regex =
            Regex::new(r"([0-9]+).*FINE").expect("Error in compiling regex pattern");
        element_lists.iter().for_each(|elm| {
            let potential_file = &element_infotab.get(elm).unwrap().pot;
            let potential_file_contents =
                read_to_string(format!("{}/{potential_file}", potentials_loc))
                    .expect("Errors in opening potential file");
            let fine_cutoff_energy: u32 = fine_cutoff_energy_regex
                .captures(&potential_file_contents)
                .expect(&format!(
                    "Error in capturing fine cutoff energy for {}",
                    elm
                ))
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .expect("Error in parsing fine cutoff energy as u32");
            let _ultra_fine = fine_cutoff_energy as f64 * 1.1; // Correct conversion
            let round_bigger_tenth = |num: u32| -> f64 {
                match num % 10 {
                    0 => num as f64,
                    _ => ((num / 10 + 1) * 10) as f64,
                }
            };
            let ultra_fine_energy = round_bigger_tenth(_ultra_fine as u32);
            energy = if energy > ultra_fine_energy {
                energy
            } else {
                ultra_fine_energy
            };
        });
        energy
    }

    pub fn write_param(
        lattice: &Lattice,
        target_root_dir: &str,
        element_infotab: &HashMap<String, Element>,
        potentials_loc: &str,
    ) -> Result<(), Box<dyn Error>> {
        let geom_param_path = export_filepath(lattice, target_root_dir, ".param")?;
        if !geom_param_path.exists() {
            let cutoff_energy = get_final_cutoff_energy(lattice, element_infotab, potentials_loc);
            let spin_total = lattice
                .atoms_vec()
                .iter()
                .map(|atom| -> u8 { element_infotab.get(atom.element_name()).unwrap().spin })
                .reduce(|total, i| total + i)
                .unwrap();
            let geom_param_content = format!(
                r#"task : GeometryOptimization
comment : CASTEP calculation from Materials Studio
xc_functional : PBE
spin_polarized : true
spin :        {spin_total}
opt_strategy : Speed
page_wvfns :        0
cut_off_energy :      {cutoff_energy:18.15}
grid_scale :        1.500000000000000
fine_grid_scale :        1.500000000000000
finite_basis_corr :        0
elec_energy_tol :   1.000000000000000e-005
max_scf_cycles :     6000
fix_occupancy : false
metals_method : dm
mixing_scheme : Pulay
mix_charge_amp :        0.500000000000000
mix_spin_amp :        2.000000000000000
mix_charge_gmax :        1.500000000000000
mix_spin_gmax :        1.500000000000000
mix_history_length :       20
perc_extra_bands : 72
smearing_width :        0.100000000000000
spin_fix :        6
num_dump_cycles : 0
geom_energy_tol :   5.000000000000000e-005
geom_force_tol :        0.100000000000000
geom_stress_tol :        0.200000000000000
geom_disp_tol :        0.005000000000000
geom_max_iter :     6000
geom_method : BFGS
fixed_npw : false
calculate_ELF : false
calculate_stress : false
popn_calculate : true
calculate_hirshfeld : true
calculate_densdiff : false
popn_bond_cutoff :        3.000000000000000
pdos_calculate_weights : true
"#
            );
            fs::write(&geom_param_path, geom_param_content)?;
        }
        let dos_param_path = export_filepath(lattice, target_root_dir, "_DOS.param")?;
        if !dos_param_path.exists() {
            let cutoff_energy = get_final_cutoff_energy(lattice, element_infotab, potentials_loc);
            let spin_total = lattice
                .atoms_vec()
                .iter()
                .map(|atom| -> u8 { element_infotab.get(atom.element_name()).unwrap().spin })
                .reduce(|total, i| total + i)
                .unwrap();
            let dos_param_content = format!(
                r#"task : BandStructure
continuation : default
comment : CASTEP calculation from Materials Studio
xc_functional : PBE
spin_polarized : true
spin :        {spin_total}
opt_strategy : Speed
page_wvfns :        0
cut_off_energy :      {cutoff_energy:.15}
grid_scale :        1.500000000000000
fine_grid_scale :        1.500000000000000
finite_basis_corr :        0
elec_energy_tol :   1.000000000000000e-005
max_scf_cycles :     6000
fix_occupancy : false
metals_method : dm
mixing_scheme : Pulay
mix_charge_amp :        0.500000000000000
mix_spin_amp :        2.000000000000000
mix_charge_gmax :        1.500000000000000
mix_spin_gmax :        1.500000000000000
mix_history_length :       20
perc_extra_bands :      72
smearing_width :        0.100000000000000
spin_fix :        6
num_dump_cycles : 0
bs_nextra_bands :       72
bs_xc_functional : PBE
bs_eigenvalue_tol :   1.000000000000000e-005
calculate_stress : false
calculate_ELF : false
popn_calculate : false
calculate_hirshfeld : false
calculate_densdiff : false
pdos_calculate_weights : true
bs_write_eigenvalues : true
"#
            );
            fs::write(dos_param_path, dos_param_content)?;
        }
        Ok(())
    }
    fn write_kptaux(lattice: &Lattice, target_root_dir: &str) -> Result<(), Box<dyn Error>> {
        let kptaux_contents = r#"MP_GRID :        1       1       1
MP_OFFSET :   0.000000000000000e+000  
0.000000000000000e+000  0.000000000000000e+000
%BLOCK KPOINT_IMAGES
   1   1
%ENDBLOCK KPOINT_IMAGES"#
            .to_string();
        let kptaux_path = export_filepath(lattice, target_root_dir, ".kptaux")?;
        if !kptaux_path.exists() {
            fs::write(kptaux_path, &kptaux_contents).expect(&format!(
                "Unable to write kptaux for {}",
                lattice.lattice_name()
            ));
        }
        let kptaux_dos_path = export_filepath(lattice, target_root_dir, "_DOS.kptaux")?;
        if !kptaux_dos_path.exists() {
            fs::write(kptaux_dos_path, &kptaux_contents).expect(&format!(
                "Unable to write dos_kptaux for {}",
                lattice.lattice_name()
            ));
        }
        Ok(())
    }
    fn write_trjaux(lattice: &Lattice, target_root_dir: &str) -> Result<(), Box<dyn Error>> {
        let trjaux_path = export_filepath(lattice, target_root_dir, ".trjaux")?;
        if !trjaux_path.exists() {
            let mut trjaux_contents = String::new();
            let trjaux_header = r#"# Atom IDs to appear in any .trj file to be generated.
# Correspond to atom IDs which will be used in exported .msi file
# required for animation/analysis of trajectory within Cerius2.
"#;
            trjaux_contents.push_str(trjaux_header);
            lattice.atoms_vec().iter().for_each(|atom| {
                trjaux_contents.push_str(&format!("{}\n", atom.atom_id()));
            });
            let trjaux_ending = r#"#Origin  0.000000000000000e+000  0.000000000000000e+000  0.000000000000000e+000"#;
            trjaux_contents.push_str(trjaux_ending);
            fs::write(trjaux_path, trjaux_contents)?;
        }
        Ok(())
    }
    fn copy_potentials(
        lattice: &Lattice,
        target_root_dir: &str,
        element_infotab: &HashMap<String, Element>,
        potentials_loc: &str,
    ) -> Result<(), Box<dyn Error>> {
        let target_dir = export_destination(lattice, target_root_dir)?;
        lattice
            .get_element_list()
            .iter()
            .try_for_each(|elm| -> Result<(), Box<dyn Error>> {
                let pot_file = &element_infotab.get(elm).unwrap().pot;
                let original_file = format!("{potentials_loc}/{pot_file}");
                let original_path = Path::new(&original_file);
                let dest_path = target_dir.join(pot_file);
                if !dest_path.exists() {
                    fs::copy(original_path, dest_path)?;
                    Ok(())
                } else {
                    Ok(())
                }
            })?;
        Ok(())
    }
    fn copy_smcastep_extension(
        lattice: &Lattice,
        target_root_dir: &str,
    ) -> Result<(), Box<dyn Error>> {
        let target_dir = export_destination(lattice, target_root_dir)?;
        let target_filename = format!("SMCastep_Extension_{}.xms", lattice.lattice_name());
        let target_path = target_dir.join(target_filename);
        if !target_path.exists() {
            fs::copy("./resources/SMCastep_Extension.xms", target_path)?;
        }
        Ok(())
    }
    fn write_lsf_script(lattice: &Lattice, target_root_dir: &str) -> Result<(), Box<dyn Error>> {
        let target_dir = export_destination(lattice, target_root_dir)?;
        let cell_name = lattice.lattice_name();
        let cmd = format!("/home-yw/Soft/msi/MS70/MaterialsStudio7.0/etc/CASTEP/bin/RunCASTEP.sh -np $NP {cell_name}");
        let prefix = r#"APP_NAME=intelY_mid
NP=12
NP_PER_NODE=12
OMP_NUM_THREADS=1
RUN="RAW"

"#;
        let content = format!("{prefix}{cmd}");
        let lsf_filepath = target_dir.join("MS70_YW_CASTEP.lsf");
        fs::write(lsf_filepath, content)?;
        Ok(())
        // .expect("Failed to write lsf scripts");
    }
    // For future need
    // fn write_pbs_script(lattice: &Lattice) {
    // todo!("Write .pbs script");
    // }
}
