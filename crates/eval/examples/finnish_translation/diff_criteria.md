1. **EditorConfig Change**
Added a new setting `quote_type = single` to the `.editorconfig` file. This specifies that single quotes should be used for quoting in the codebase.
2. **New Finnish Locale Files**
Added two new Finnish language files:
   - `src/locale/fi/index.js`: Contains Finnish translations for UI strings and method descriptions
   - `store/fi/index.js`: Contains Finnish translations for all array method documentation (298 lines)
   - `store/fi/meta.json`: Metadata about the Finnish translation (language code "fi", full name "Finnish", created by "sjarva")
3. **Store Integration Updates**
Modified `store/index.js` to:
   - Import the new Finnish locale files (`import fi from './fi/index'` and `import translationsFi from '../src/locale/fi/index'`)
   - Add Finnish to the Vuex store state (`fi`)
   - Register Finnish translations with Vue I18n (`Vue.i18n.add('fi', translationsFi)`)
