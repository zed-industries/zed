import {
    NO_ACTIVE_ERROR,
    NO_INACTIVE_OR_BASE_ERROR,
    toggleable,
} from "./toggle"
import { describe, it, expect } from "vitest"

describe("toggleable", () => {
    it("creates a Toggleable<Element> with base properties and states", () => {
        const result = toggleable({
            base: { background: "#000000", color: "#CCCCCC" },
            state: {
                active: { color: "#FFFFFF" },
            },
        })

        expect(result).toEqual({
            inactive: { background: "#000000", color: "#CCCCCC" },
            active: { background: "#000000", color: "#FFFFFF" },
        })
    })

    it("creates a Toggleable<Element> with no base properties", () => {
        const result = toggleable({
            state: {
                inactive: { background: "#000000", color: "#CCCCCC" },
                active: { background: "#000000", color: "#FFFFFF" },
            },
        })

        expect(result).toEqual({
            inactive: { background: "#000000", color: "#CCCCCC" },
            active: { background: "#000000", color: "#FFFFFF" },
        })
    })

    it("throws error when both inactive and base are missing", () => {
        const state = {
            active: { background: "#000000", color: "#FFFFFF" },
        }

        expect(() => toggleable({ state })).toThrow(NO_INACTIVE_OR_BASE_ERROR)
    })

    it("throws error when no active state is present", () => {
        const state = {
            inactive: { background: "#000000", color: "#CCCCCC" },
        }

        expect(() => toggleable({ state })).toThrow(NO_ACTIVE_ERROR)
    })
})
