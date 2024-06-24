# CK3OOP Tool

CK3OOP is a simple tool for sorting / managing / developing mods in Crusader Kings III.

## Note
This is in active development.

## Build Scripts and GitHub Actions Workflow Usage

The build process and GitHub Actions workflows have been updated to improve efficiency and maintainability. Here's how to use the updated tooling:

### Using the `justfile` for Build and Test Operations

The `justfile` now consolidates related commands to reduce redundancy and improve readability. It introduces variables for commonly used paths and values, simplifying command updates. Here are some examples of how to use it:

- To initialize the build environment, run `just init`.
- To download the required webdriver, run `just download-webdriver`.
- To build the end-to-end tests, run `just build-e2e`.
- To run the end-to-end tests, run `just test-e2e`.

### GitHub Actions Workflows

The GitHub Actions workflows have been consolidated into a single `publish.yaml` file, which handles both NPM package and Tauri artifact publishing. This reduces redundancy and improves readability. The workflow is triggered on specific events, and conditional steps ensure the correct publishing actions are taken based on the context.

### Troubleshooting Common Issues

If you encounter issues with the build process, consider the following troubleshooting steps:

- Ensure all dependencies are correctly installed by running `npm install` or the equivalent command for your setup.
- Check the `justfile` and GitHub Actions workflow logs for any error messages that might indicate what went wrong.
- Verify that all environment variables and paths used in the `justfile` and workflows are correctly set.

For more detailed instructions and troubleshooting tips, refer to the documentation in the repository's wiki or issue tracker.

## Old Outdated Readme
<details><summary><a href="https://Example.com">Spoiler</a></summary>
## Features

- Reads and parses the `dlc_load.json` file to determine enabled mods.
- Processes mod files and extracts relevant information.
- Supports running custom rules on the parsed data.

## Examples
In the [example](./example) directory, you can find an example of how to use the tool.

Here is an example of a rule that checks if any `on_action` events in the mod conflict with the base game:
```javascript
const path = require("path");
const fs = require("fs");
const _ = require('lodash');
const utils = require('./utils');

function conflictOnAction(
        context,
        {
           parser,
           gameFiles,
           enabledMods,
           enabledFiles,
        },
) {

   // Directory where on_action files are located
   const onActionDir = "\\common\\on_action";

   // We want to find all on_actions files in the base game
   const vanillaOnActionFiles = gameFiles.filter(gameFile => gameFile.dir === onActionDir);

   // Get all on_action events in the base game
   let vanillaOnActionEvents = vanillaOnActionFiles.map((f) => {
      let fileContent = fs.readFileSync(f.fullPath, "utf-8");
      let parsed = parser.parseText(fileContent);
      return Object.keys(parsed);
   }).flat();

   // For each mods check if any of its on_actions conflict with the base game
   // That mean that it has the same key and overwrites "trigger" or "effect"
   enabledMods.forEach((mod) => {

      // Get all on_action files in the mod
      let modOnActionFiles = mod.files.filter(file => file.dir === onActionDir);

      // Loop through all on_action files
      let modOnActions = [];
      modOnActionFiles.forEach(onActionFile => {

         // Parse and push all keys to the list
         let parsed = utils.tryParse(parser, onActionFile.fullPath);
         modOnActions.push(...Object.keys(parsed));

         // Iterate through all keys in the on_action file
         Object.keys(parsed).forEach((key)=>{

            // If the key is in the base game, check if it conflicts
            if(vanillaOnActionEvents.includes(key)) {
               if (parsed[key].effect !== undefined) {
                  throw new Error(`Conflict in ${mod.name} on_action: ${key} has an effect`);
               }
               if (parsed[key].trigger !== undefined) {
                  throw new Error(`Conflict in ${mod.name} on_action: ${key} has a trigger`);
               }
               // Log that it's correct
               console.log(`No conflict in ${mod.name} on_action: ${key}`);
            }
         })
      })

   })
   return context;
}

conflictOnAction.name = "loadOrderRule";
module.exports = conflictOnAction;
```

## Installation

To use CK3OOP, you need to have Node.js installed on your system. You can install Node.js from the [official website](https://nodejs.org/).

1. Clone this repository:
    ```sh
    git clone https://github.com/bukowa/ck3oop.git
    cd ck3oop
    ```

2. Install the necessary dependencies:
    ```sh
    npm install
    ```

## Usage
```bash
> node main.js example

Working in directory: C:\Users\buk\WebstormProjects\ck3oop\example
Using config file: C:\Users\buk\WebstormProjects\ck3oop\example\ck3oop.json
Mod directory: C:\Users\buk\Documents\Paradox Interactive\Crusader Kings III
Reading dlc_load.json...
[
  'mod/ugc_2273832430.mod',
  'mod/ugc_2220098919.mod',
  'mod/ugc_2507209632.mod',
  'mod/ugc_2996881191.mod',
  'mod/ugc_2829397295.mod',
  'mod/ugc_2553043828.mod',
  'mod/ugc_2818979385.mod',
  'mod/test.mod'
]
Creating enabled mods...
[
  'Regional Immersion and Cultural Enrichment (RICE)',
  'Community Flavor Pack',
  'Ethnicities and Portraits Expanded',
  'CFP + EPE Compatibility Patch',
  'Culture Expanded',
  'RICE + EPE Compatibility Patch',
  'Unit Pack Expanded',
  'test'
]
Reading enabled rules...
[ 'parsingRule', 'saveargsasjson' ]
Reading files of mod: Regional Immersion and Cultural Enrichment (RICE)
Reading files of mod: Community Flavor Pack
Reading files of mod: Ethnicities and Portraits Expanded
Reading files of mod: CFP + EPE Compatibility Patch
Reading files of mod: Culture Expanded
Reading files of mod: RICE + EPE Compatibility Patch
Reading files of mod: Unit Pack Expanded
Reading files of mod: test
Reading files of the base game...
Creating load resolution order...
Running rules...
Running rule: parsingRule
{
  filePath: 'C:\\Users\\buk\\Documents\\Paradox Interactive\\Crusader Kings III\\mod\\ugc_2507209632.mod',
  fileName: 'mod/ugc_2507209632.mod',
  version: '1.0',
  name: 'Ethnicities and Portraits Expanded',
  picture: null,
  supported_version: '1.12.5',
  path: 'C:/Program Files (x86)/Steam/steamapps/workshop/content/1158310/2507209632',
  remote_file_id: '2507209632',
  resolvedPath: 'C:\\Program Files (x86)\\Steam\\steamapps\\workshop\\content\\1158310\\2507209632',
  files: null
}
{
  fullPath: 'C:\\Program Files (x86)\\Steam\\steamapps\\workshop\\content\\1158310\\2507209632\\common\\culture\\cultures\\00_akan.txt',
  path: '\\common\\culture\\cultures\\00_akan.txt',
  dir: '\\common\\culture\\cultures',
  file: '00_akan.txt',
  stats: null
}
{
  akan: {
    color: 'akan',
    ethos: 'ethos_egalitarian',
    heritage: 'heritage_akan',
    language: 'language_kwa',
    martial_custom: 'martial_custom_equal',
    traditions: [
      'tradition_female_only_inheritance',
      'tradition_mystical_ancestors',
      'tradition_parochialism',
      'tradition_bush_hunting'
    ],
    name_list: 'name_list_akan',
    coa_gfx: [ 'west_african_group_coa_gfx' ],
    building_gfx: [ 'african_building_gfx', 'mena_building_gfx' ],
    clothing_gfx: [ 'african_clothing_gfx' ],
    unit_gfx: [ 'central_african_unit_gfx', 'sub_sahran_unit_gfx' ],
    ethnicities: { '15': [Array] }
  },
  kru: {
    color: 'kru',
    ethos: 'ethos_stoic',
    heritage: 'heritage_akan',
    language: 'language_kru',
    martial_custom: 'martial_custom_male_only',
    traditions: [
      'tradition_fishermen',
      'tradition_practiced_pirates',
      'tradition_bush_hunting'
    ],
    name_list: 'name_list_kru',
    coa_gfx: [ 'west_african_group_coa_gfx' ],
    building_gfx: [ 'african_building_gfx', 'mena_building_gfx' ],
    clothing_gfx: [ 'african_clothing_gfx' ],
    unit_gfx: [ 'central_african_unit_gfx', 'sub_sahran_unit_gfx' ],
    ethnicities: { '15': [Array] }
  },
  guan: {
    color: { hsv: [Array] },
    ethos: 'ethos_communal',
    heritage: 'heritage_akan',
    language: 'language_kwa',
    martial_custom: 'martial_custom_equal',
    traditions: [
      'tradition_dryland_dwellers',
      'tradition_festivities',
      'tradition_zealous_people',
      'tradition_bush_hunting'
    ],
    name_list: 'name_list_guan',
    coa_gfx: [ 'west_african_group_coa_gfx' ],
    building_gfx: [ 'african_building_gfx', 'mena_building_gfx' ],
    clothing_gfx: [ 'african_clothing_gfx' ],
    unit_gfx: [ 'central_african_unit_gfx', 'sub_sahran_unit_gfx' ],
    ethnicities: { '15': [Array] }
  }
}
Running rule: saveargsasjson

Process finished with exit code 0
```

</details>
