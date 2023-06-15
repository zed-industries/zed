import { configDefaults, defineConfig } from 'vitest/config'

export default defineConfig({
    test: {
        exclude: [...configDefaults.exclude, 'target/*'],
        include: ['src/**/*.{spec,test}.ts'],
    },
})
