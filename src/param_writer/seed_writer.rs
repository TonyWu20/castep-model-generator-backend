use std::{
    ffi::OsString,
    fs::{self, create_dir_all},
    io,
    marker::PhantomData,
    path::PathBuf,
};

use crate::{
    builder_typestate::{No, ToAssign, Yes},
    lattice::LatticeModel,
    model_type::cell::CellModel,
};

use super::{
    castep_param::{CastepParam, Task},
    ms_aux_files::MsAuxWriter,
};

/// Struct to present seed files export.
/// The `'a` lifetime is the lifetime for the reference to the cell.`
#[derive(Debug)]
pub struct SeedWriter<'a, T>
where
    T: Task,
{
    cell: &'a LatticeModel<CellModel>,
    param: CastepParam<T>,
    seed_name: &'a str,
    export_loc: PathBuf,
    potential_loc: PathBuf,
}

impl<'a, T> SeedWriter<'a, T>
where
    T: Task,
{
    pub fn build(cell: &'a LatticeModel<CellModel>) -> SeedWriterBuilder<'a, T, No> {
        SeedWriterBuilder::<T, No>::new(cell)
    }
    fn path_builder(&self, extension: &str) -> Result<PathBuf, io::Error> {
        let dir_name = format!("{}_{}", self.seed_name, "opt");
        let dir_loc: OsString = self.export_loc.clone().into();
        let export_loc = PathBuf::from(dir_loc).join(&dir_name);
        create_dir_all(&export_loc)?;
        let filename = format!("{}{}", self.seed_name, extension);
        Ok(export_loc.join(filename))
    }
    pub fn write_seed_files(&self) -> Result<(), io::Error> {
        let ms_param = MsAuxWriter::build(self.seed_name, &self.export_loc)
            .with_kptaux(self.cell.build_kptaux())
            .with_trjaux(self.cell.build_trjaux())
            .with_potentials_loc(&self.potential_loc)
            .build();
        ms_param.write_kptaux()?;
        ms_param.write_trjaux()?;
        let param_path = self.path_builder(".param")?;
        fs::write(param_path, format!("{}", self.param))?;
        let cell_path = self.path_builder(".cell")?;
        fs::write(cell_path, self.cell.cell_export())?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct SeedWriterBuilder<'a, T, WithPotentialLoc>
where
    T: Task,
    WithPotentialLoc: ToAssign,
{
    cell: &'a LatticeModel<CellModel>,
    param: Option<CastepParam<T>>,
    seed_name: &'a str,
    export_loc: PathBuf,
    potential_loc: PathBuf,
    potential_set_state: PhantomData<WithPotentialLoc>,
}

impl<'a, T, P> SeedWriterBuilder<'a, T, P>
where
    T: Task,
    P: ToAssign,
{
    pub fn new(cell: &'a LatticeModel<CellModel>) -> SeedWriterBuilder<T, No> {
        SeedWriterBuilder {
            cell,
            param: None,
            seed_name: "",
            export_loc: PathBuf::new(),
            potential_loc: PathBuf::new(),
            potential_set_state: PhantomData,
        }
    }
    pub fn with_potential_loc(self, potential_loc: &'a str) -> SeedWriterBuilder<T, Yes> {
        let new_potential_loc = self.potential_loc.join(potential_loc);
        let Self {
            cell,
            param,
            seed_name,
            export_loc,
            potential_loc: _,
            potential_set_state: _,
        } = self;
        SeedWriterBuilder {
            cell,
            param,
            seed_name,
            export_loc,
            potential_loc: new_potential_loc,
            potential_set_state: PhantomData,
        }
    }
    pub fn with_export_loc(self, export_loc: &'a str) -> SeedWriterBuilder<T, P> {
        let new_export_loc = self.export_loc.join(export_loc);
        let Self {
            cell,
            param,
            seed_name,
            export_loc: _,
            potential_loc,
            potential_set_state,
        } = self;
        SeedWriterBuilder {
            cell,
            param,
            seed_name,
            export_loc: new_export_loc,
            potential_loc,
            potential_set_state,
        }
    }
    pub fn with_seed_name(self, new_seed_name: &'a str) -> SeedWriterBuilder<T, P> {
        let Self {
            cell,
            param,
            seed_name: _,
            export_loc,
            potential_loc,
            potential_set_state,
        } = self;
        SeedWriterBuilder {
            cell,
            param,
            seed_name: new_seed_name,
            export_loc,
            potential_loc,
            potential_set_state,
        }
    }
}

impl<'a, T> SeedWriterBuilder<'a, T, Yes>
where
    T: Task,
{
    pub fn build(self) -> SeedWriter<'a, T> {
        let param = CastepParam::<T>::build()
            .with_spin_total(self.cell.spin_total())
            .with_cut_off_energy(
                self.cell
                    .get_final_cutoff_energy(self.potential_loc.to_str().unwrap()),
            )
            .build();
        let Self {
            cell,
            param: _,
            seed_name,
            export_loc,
            potential_loc,
            potential_set_state: _,
        } = self;
        SeedWriter {
            cell,
            param,
            seed_name,
            export_loc,
            potential_loc,
        }
    }
}
