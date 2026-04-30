```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_KEYMAP_SIZE 256

typedef struct {
    char key[10];
    char command[100];
} KeymapEntry;

void printKeymap(const KeymapEntry *keymap, int size) {
    for (int i = 0; i < size; ++i) {
        printf("Key: %s, Command: %s\n", keymap[i].key, keymap[i].command);
    }
}

int main() {
    KeymapEntry *keymap = (KeymapEntry *)malloc(MAX_KEYMAP_SIZE * sizeof(KeymapEntry));
    if (keymap == NULL) {
        fprintf(stderr, "Memory allocation failed\n");
        return 1;
    }

    // Initialize keymap entries
    strncpy(keymap[0].key, "F1", sizeof(keymap[0].key));
    strncpy(keymap[0].command, "Open File", sizeof(keymap[0].command));
    strncpy(keymap[1].key, "F2", sizeof(keymap[1].key));
    strncpy(keymap[1].command, "Save File", sizeof(keymap[1].command));
    strncpy(keymap[2].key, "F3", sizeof(keymap[2].key));
    strncpy(keymap[2].command, "Find", sizeof(keymap[2].command));

    printKeymap(keymap, 3);

    free(keymap);
    return 0;
}
```