# Castep-model-generator-backend
A rust-written backend lib for edit and generation of lattice 3D-models compatible with `CASTEP` and `Materials Studio`.

## Related projects
1. [castep-model-core](https://github.com/TonyWu20/castep-model-core.git)

## Introduction

This library is 100% written in `rust`. Currently, it has the following features:
1. Place any molecules into a lattice, with necessary information provided:
    1. The number and identities of atoms used to coordinate with the lattice.
    2. The geometry information to determine the molecule direction in 3d space.
2. Export the seedfiles for `CASTEP` task, supported by [castep-model-core](https://github.com/TonyWu20/castep-model-core.git)
3. Parallel batch processing for model edit, molecule placement onto the lattice, and exports, supported by `rayon`.
    - The detailed workflow for iteration is implemented by yourself.
4. Configurable with `YAML` format files.

## Configuration with `YAML` files

This lib is designed to be controlled with specific `YAML` files.
You can also hardcode the necessary information in your crate.

### Project-level: The `YAML` file for project level control provides the following fields:
1. `base_model_loc`: path to the base model `msi` file. This lattice model is used for the consequent batch editing and adsorbate placement.
2. `element_table_loc`: path to the element table `YAML` file. It will include the elements that will be involved in your project.
3. `adsorbate_table_loc`: path to the adsorbate information `YAML` file. It records the necessary information to deal with the placement of adsorbates.
4. `potentials_loc`: path to the `CASTEP` potential files directory.
5. `export_loc`: path to export your results.
6. `coord_sites`: An array recording the target coordination sites that will be used in your lattice models. It has the following format for example:
    ```
    coord_sites:
        - name: c1
        atom_id: 41
        - name: c2
        atom_id: 42
    ```
    The `name` field is for naming your exported models. The `atom_id` field is consistent with the id in `msi` file, which is one-indexed.
7. `coord_cases`: You will describe the possible coordination cases between the lattice and adsorbates. Currently, the library is designed to handle cases with coordination atoms up to two. For example:
    ```
    coord_cases:
    - name: double
        cases: [[41,42], [42,54], [54,53], [53,52], [41, 40], [41, 73], [42, 73]]
    - name: single
        cases: [[41, null], [42, null], [54, null], [53, null], [52, null], [40, null], [73, null]]
    ```
### Adsorbate information
This `YAML` file describes the necessary information about the adsorbate, which is key to the placement workflow of adsorbates onto the lattice.

Example structures:
```
directory: C2_pathways_ads
Adsorbates:
  - name: CO
    coordAtomIds: [1]
    stemAtomIds: [1,2]
    # planeAtomIds missing since it has only two atoms
    # plane_angle missing since it has only two atoms
    stem_coord_angle: 90.0
    bSym: false
    upperAtomId: 2
    atomNums: 2
    pathName: ethylene
  - name: CHO
    coordAtomIds: [1]
    stemAtomIds: [2, 3]
    planeAtomIds: [1, 2, 3]
    plane_angle: 90.0
    stem_coord_angle: 90.0
    bSym: false
    upperAtomId: 2
    atomNums: 3
    pathName: ethylene

```
The `directory` field tells the code where to locate the adsorbate `msi` files. The `pathName` field helps the code to find the adsorbate: Currently this lib is designed to find the adsorbate according to the path: `./{directory}/{pathName}_path/{adsorbate_name}.msi`.


