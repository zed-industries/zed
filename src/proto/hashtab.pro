/* hashtab.c */
void hash_init(hashtab_T *ht);
int check_hashtab_frozen(hashtab_T *ht, char *command);
void hash_clear(hashtab_T *ht);
void hash_clear_all(hashtab_T *ht, int off);
hashitem_T *hash_find(hashtab_T *ht, char_u *key);
hashitem_T *hash_lookup(hashtab_T *ht, char_u *key, hash_T hash);
void hash_debug_results(void);
int hash_add(hashtab_T *ht, char_u *key, char *command);
int hash_add_item(hashtab_T *ht, hashitem_T *hi, char_u *key, hash_T hash);
int hash_remove(hashtab_T *ht, hashitem_T *hi, char *command);
void hash_lock(hashtab_T *ht);
void hash_lock_size(hashtab_T *ht, int size);
void hash_unlock(hashtab_T *ht);
hash_T hash_hash(char_u *key);
/* vim: set ft=c : */
