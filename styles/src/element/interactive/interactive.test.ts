import { NOT_ENOUGH_STATES_ERROR, NO_DEFAULT_OR_BASE_ERROR, interactive } from './interactive'
import { describe, it, expect } from 'vitest'

describe('interactive', () => {

    it('creates an Interactive<Element> with base properties and states', () => {

        const result = interactive({
            base: { fontSize: 10, color: '#FFFFFF' },
            state: {
                hovered: { color: '#EEEEEE' },
                clicked: { color: '#CCCCCC' },
            }
        })

        expect(result).toEqual({
            default: { color: '#FFFFFF', fontSize: 10 },
            hovered: { color: '#EEEEEE', fontSize: 10 },
            clicked: { color: '#CCCCCC', fontSize: 10 },
        })
    })

    it('creates an Interactive<Element> with no base properties', () => {

        const result = interactive({
            state: {
                default: { color: '#FFFFFF', fontSize: 10 },
                hovered: { color: '#EEEEEE' },
                clicked: { color: '#CCCCCC' },
            }
        })

        expect(result).toEqual({
            default: { color: '#FFFFFF', fontSize: 10 },
            hovered: { color: '#EEEEEE', fontSize: 10 },
            clicked: { color: '#CCCCCC', fontSize: 10 },
        })
    })

    it('throws error when both default and base are missing', () => {
        const state = {
            hovered: { color: 'blue' },
        }

        expect(() => interactive({ state })).toThrow(
            NO_DEFAULT_OR_BASE_ERROR
        )
    })

    it('throws error when no other state besides default is present', () => {
        const state = {
            default: { fontSize: 10 },
        }

        expect(() => interactive({ state })).toThrow(
            NOT_ENOUGH_STATES_ERROR
        )
    })
})
