# Castep-model-generator-backend
A rust-written backend lib for edit and generation of lattice 3D-models compatible with `CASTEP` and `Materials Studio`.

## Introduction

This library is 100% written in `rust`. Currently, it has the following features:
1. I/O of existing `.msi` format model files. However the format for atom parsing is strict for the moment.
2. Edit atoms and lattice information in the model.
    1. Edit the element information, atom ID, xyz coordinates of target atom.
    2. Read/Write the lattice vectors.
3. Geometry transformation of the model:
    1. Translation to desired positions
    2. Rotate by axis-angle definition.
4. Place any molecules into a lattice, with necessary information provided:
    1. The number and identities of atoms used to coordinate with the lattice.
    2. The geometry information to determine the molecule direction in 3d space.
5. Export the seedfiles for `CASTEP` task, including:
    - `*.cell` - necessary file for `CASTEP` task. Can be visualized in many simple and lightweight model viewing software. E.g. `VESTA`.
    - `*.trjaux`, `*.kptaux`
    - `*.param`
    - `*.msi` - can be visualized in `Materials Studio`.
    - Copy potential files used for `CASTEP` standalone mode. (Potential files are not provided and included in this repository and library)
    - Miscellaneous files.
    - Auto-generation of a `perl` script to instruct the `Materials Studio` to generate `.xsd` from `.msi`.
6. Parallel batch processing for model edit, molecule placement onto the lattice, and exports, supported by `rayon`.
    - The detailed workflow for iteration is implemented by yourself.
7. Configure with `YAML` format files.

## Configuration with `YAML` files

This lib is designed to be controlled with specific `YAML` files.

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
### Element table
This `YAML` describes the necessary information of elements, for writing `cell` and finding corresponding potential files.

Example structures:
```
Element_info:
  - element: C
    atomic_num: 6
    LCAO: 2
    mass: 12.0109996796
    pot: C_00PBE.usp
    spin: 0
  - element: H
    atomic_num: 0
    LCAO: 1
    mass: 1.0080000162
    pot: H_00PBE.usp
    spin: 0
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
    planeAtomIds: [1,2,2]
    vertical: true
    bSym: false
    upperAtomId: 2
    atomNums: 2
    pathName: ethylene
  - name: CHO
    coordAtomIds: [1]
    stemAtomIds: [2, 3]
    planeAtomIds: [1, 2, 3]
    vertical: true
    bSym: false
    upperAtomId: 2
    atomNums: 3
    pathName: ethylene

```
The `directory` field tells the code where to locate the adsorbate `msi` files. The `pathName` field helps the code to find the adsorbate: Currently this lib is designed to find the adsorbate according to the path: `./{directory}/{pathName}_path/{adsorbate_name}.msi`.


