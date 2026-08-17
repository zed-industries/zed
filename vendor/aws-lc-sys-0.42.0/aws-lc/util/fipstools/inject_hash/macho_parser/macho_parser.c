// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <stdint.h>

#include "common.h"
#include "macho_parser.h"

#define TEXT_INDEX 0
#define CONST_INDEX 1
#define SYMTABLE_INDEX 2
#define STRTABLE_INDEX 3

// Documentation for the Mach-O structs can be found in macho-o/loader.h and mach-o/nlist.h
int read_macho_file(const char *filename, machofile *macho) {
    FILE *file = NULL;
    struct load_command *load_commands = NULL;
    uint32_t bytes_read;
    int ret = 0;

    file = fopen(filename, "rb");
    if (file == NULL) {
        LOG_ERROR("Error opening file %s", filename);
        goto end;
    }

    bytes_read = fread(&macho->macho_header, 1, sizeof(struct mach_header_64), file);
    if (bytes_read != sizeof(struct mach_header_64)) {
        LOG_ERROR("Error reading macho_header from file %s", filename);
        goto end;
    }
    if (macho->macho_header.magic != MH_MAGIC_64) {
        LOG_ERROR("File is not a 64-bit Mach-O file");
        goto end;
    }

    load_commands = malloc(macho->macho_header.sizeofcmds);
    if (load_commands == NULL) {
        LOG_ERROR("Error allocating memory for load_commands");
        goto end;
    }
    bytes_read = fread(load_commands, 1, macho->macho_header.sizeofcmds, file);
    if (bytes_read != macho->macho_header.sizeofcmds) {
        LOG_ERROR("Error reading load commands from file %s", filename);
        goto end;
    }

    // We're only looking for __text, __const in the __TEXT segment, and the string & symbol tables
    macho->num_sections = 4;
    macho->sections = malloc(macho->num_sections * sizeof(section_info));
    if (macho->sections == NULL) {
        LOG_ERROR("Error allocating memory for macho sections");
    }

    int text_found = 0;
    int const_found = 0;
    int symtab_found = 0;

    // mach-o/loader.h explains that cmdsize (and by extension sizeofcmds) must be a multiple of 8 on 64-bit systems. struct load_command will always be 8 bytes.
    for (size_t i = 0; i < macho->macho_header.sizeofcmds / sizeof(struct load_command); i += load_commands[i].cmdsize / sizeof(struct load_command)) {
        if (load_commands[i].cmd == LC_SEGMENT_64) {
            struct segment_command_64 *segment = (struct segment_command_64 *)&load_commands[i];
            if (strcmp(segment->segname, "__TEXT") == 0) {
                struct section_64 *sections = (struct section_64 *)&segment[1];
                for (size_t j = 0; j < segment->nsects; j++) {
                    if (strcmp(sections[j].sectname, "__text") == 0) {
                        if (text_found == 1) {
                            LOG_ERROR("Duplicate __text section found");
                            goto end;
                        }
                        macho->sections[TEXT_INDEX].offset = sections[j].offset;
                        macho->sections[TEXT_INDEX].size = sections[j].size;
                        strcpy(macho->sections[TEXT_INDEX].name, sections[j].sectname);
                        text_found = 1;
                    } else if (strcmp(sections[j].sectname, "__const") == 0) {
                        if (const_found == 1) {
                            LOG_ERROR("Duplicate __const section found");
                            goto end;
                        }
                        macho->sections[CONST_INDEX].offset = sections[j].offset;
                        macho->sections[CONST_INDEX].size = sections[j].size;
                        strcpy(macho->sections[CONST_INDEX].name, sections[j].sectname);
                        const_found = 1;
                    }
                }
            }
        } else if (load_commands[i].cmd == LC_SYMTAB) {
            if (symtab_found == 1) {
                LOG_ERROR("Duplicate symbol and string tables found");
                goto end;
            }
            struct symtab_command *symtab = (struct symtab_command *)&load_commands[i];
            macho->sections[SYMTABLE_INDEX].offset = symtab->symoff;
            macho->sections[SYMTABLE_INDEX].size = symtab->nsyms * sizeof(struct nlist_64);
            strcpy(macho->sections[SYMTABLE_INDEX].name, "__symbol_table");
            macho->sections[STRTABLE_INDEX].offset = symtab->stroff;
            macho->sections[STRTABLE_INDEX].size = symtab->strsize;
            strcpy(macho->sections[STRTABLE_INDEX].name, "__string_table");
            symtab_found = 1;
        }
    }

    ret = 1;
end:
    free(load_commands);
    if (file != NULL) {
        fclose(file);
    }
    return ret;
}

void free_macho_file(machofile *macho) {
    free(macho->sections);
    free(macho);
    macho = NULL;
}

uint8_t* get_macho_section_data(const char *filename, machofile *macho, const char *section_name, size_t *size, uint32_t *offset) {
    FILE *file = NULL;
    uint8_t *ret = NULL;
    uint32_t bytes_read;

    file = fopen(filename, "rb");
    if (file == NULL) {
        LOG_ERROR("Error opening file %s", filename);
        goto end;
    }

    int section_index;
    if (strcmp(section_name, "__text") == 0) {
        section_index = TEXT_INDEX;
    } else if (strcmp(section_name, "__const") == 0) {
        section_index = CONST_INDEX;
    } else if (strcmp(section_name, "__symbol_table") == 0) {
        section_index = SYMTABLE_INDEX;
    } else if (strcmp(section_name, "__string_table") == 0) {
        section_index = STRTABLE_INDEX;
    } else {
        LOG_ERROR("Getting invalid macho section data %s", section_name);
        goto end;
    }

    uint8_t *section_data = malloc(macho->sections[section_index].size);
    if (section_data == NULL) {
        LOG_ERROR("Error allocating memory for section data");
        goto end;
    }

    if (fseek(file, macho->sections[section_index].offset, SEEK_SET) != 0) {
        free(section_data);
        LOG_ERROR("Failed to seek in file %s", filename);
        goto end;
    }
    bytes_read = fread(section_data, 1, macho->sections[section_index].size, file);
    if (bytes_read != macho->sections[section_index].size) {
        free(section_data);
        LOG_ERROR("Error reading section data from file %s", filename);
        goto end;
    }

    if (size != NULL) {
        *size = macho->sections[section_index].size;
    }
    if (offset != NULL) {
        *offset = macho->sections[section_index].offset;
    }

    ret = section_data;

end:
    if (file != NULL) {
        fclose(file);
    }
    return ret;
}

uint32_t find_macho_symbol_index(uint8_t *symbol_table_data, size_t symbol_table_size, uint8_t *string_table_data, size_t string_table_size, const char *symbol_name, uint32_t *base) {
    char* string_table = NULL;
    uint32_t ret = 0;

    if (symbol_table_data == NULL || string_table_data == NULL) {
        LOG_ERROR("Symbol and string table pointers cannot be null to find the symbol index");
        goto end;
    }

    string_table = malloc(string_table_size);
    if (string_table == NULL) {
        LOG_ERROR("Error allocating memory for string table");
        goto end;
    }
    memcpy(string_table, string_table_data, string_table_size);

    int found = 0;
    size_t index = 0;
    for (size_t i = 0; i < symbol_table_size / sizeof(struct nlist_64); i++) {
        struct nlist_64 *symbol = (struct nlist_64 *)(symbol_table_data + i * sizeof(struct nlist_64));
        if (strcmp(symbol_name, &string_table[symbol->n_un.n_strx]) == 0) {
            if (found == 0) {
                index = symbol->n_value;
                found = 1;
            } else {
                LOG_ERROR("Duplicate symbol %s found", symbol_name);
                goto end;
            }
        }
    }
    if (found == 0) {
        LOG_ERROR("Requested symbol %s not found", symbol_name);
        goto end;
    }
    if (base != NULL) {
        index = index - *base;
    }
    ret = index;

end:
    free(string_table);
    return ret;
}
