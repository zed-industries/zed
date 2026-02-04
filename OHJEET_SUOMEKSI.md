# Ohjeet: "home" oletusbranch ja erotettu Zed-päivityksistä

## Lyhyt vastaus

Tämä PR luo `home` branchin ja konfiguraation pitämään se erillään Zed-projektinalkuperäisistä päivityksistä.

## Mitä tehtiin

1. ✅ Luotiin `home` branch
2. ✅ Luotiin `BRANCH_STRATEGY.md` dokumentaatio (englanniksi)
3. ✅ Lisättiin GitHub Actions tarkistus estämään vahingossa tapahtuva upstream-merge
4. ✅ Luotiin `script/setup-repo.sh` apuskripti
5. ✅ Päivitettiin README viittaamaan branch-strategiaan

## Oletusbranchin asettaminen GitHubissa

**Tärkeää**: Oletusbranchin vaihtaminen täytyy tehdä GitHubin asetuksissa:

1. Mene suoraan: https://github.com/Jounikka1918/zed_cachy_experiment/settings/branches
   (Vaatii admin-oikeudet)
2. Kohdassa "Default branch", klikkaa kynä-ikonia tai ⟷ painiketta
3. Valitse `home` pudotusvalikosta
4. Klikkaa **Update** ja vahvista

Tarkemmat ohjeet: [SETUP_DEFAULT_BRANCH.md](./SETUP_DEFAULT_BRANCH.md)

## Kuinka pitää "home" erillään Zed-päivityksistä

### Vaihtoehto 1: Ei upstream remotea (Suositus)

Yksinkertaisin tapa on olla lisäämättä Zedin alkuperäistä repositorya remoteksi:

```bash
# Tarkista nykyiset remotet
git remote -v

# Jos upstream on olemassa, poista se
git remote remove upstream
```

### Vaihtoehto 2: Lisää upstream vain tarvittaessa

Jos joskus haluat cherry-pickata yksittäisiä ominaisuuksia Zedistä:

```bash
# Lisää upstream remote (vain tarvittaessa)
git remote add upstream https://github.com/zed-industries/zed.git

# Hae upstream muutokset (ei mergettä)
git fetch upstream

# Cherry-pick yksittäisiä committeja
git cherry-pick <commit-hash>
```

**⚠️ Tärkeää**: Älä koskaan tee `git merge upstream/main` tai `git pull upstream main` - tämä toisi kaikki upstream-muutokset.

### Automaattinen tarkistus

GitHub Actions tarkistaa automaattisesti pull requestit ja varoittaa, jos ne sisältävät upstream-mergejä.

## Työskentelymalli

1. **Työskentele aina `home` branchissa tai sen haaroista**:
   ```bash
   git checkout home
   git checkout -b feature/uusi-ominaisuus
   ```

2. **Pull requestit kohteena `home`**:
   - Base branch: `home`
   - Compare branch: `feature/uusi-ominaisuus`

3. **Älä koskaan mergettä upstream/main branchia**

## Lisätietoja

Katso `BRANCH_STRATEGY.md` yksityiskohtaiset ohjeet (englanniksi).

---

**Päivitetty**: Helmikuu 2026
