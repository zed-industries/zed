# Instructions: Setting 'home' as Default Branch on GitHub

After this PR is merged, follow these steps to complete the setup:

## Step 1: Create the 'home' branch on GitHub (if needed)

If the `home` branch doesn't exist on GitHub yet:

1. From the branch that was created by this PR, create a new branch called `home`:
   ```bash
   git checkout <this-pr-branch>
   git checkout -b home
   git push origin home
   ```

OR manually on GitHub:
1. Go to https://github.com/Jounikka1918/zed_cachy_experiment
2. Click the branch dropdown
3. Type "home" and create the branch

## Step 2: Set 'home' as Default Branch

1. Go to https://github.com/Jounikka1918/zed_cachy_experiment/settings/branches
   (Requires admin/write access to the repository)

2. Under "Default branch", you'll see the current default branch

3. Click the ⟷ (switch branches) button or pencil icon next to the default branch

4. In the dropdown, select `home`

5. Click "Update" button

6. Confirm the change by clicking "I understand, update the default branch"

## Step 3: Verify the Change

1. Go back to the main repository page: https://github.com/Jounikka1918/zed_cachy_experiment

2. The branch dropdown should now show `home` as the default

3. New clones will automatically check out the `home` branch

4. New PRs will default to targeting the `home` branch

## Step 4: Optional - Remove upstream remote (if exists)

To prevent accidental syncing with Zed upstream:

```bash
# Check if upstream remote exists
git remote -v

# If it shows a zed-industries remote, remove it
git remote remove upstream
```

OR run the provided setup script:
```bash
./script/setup-repo.sh
```

## Troubleshooting

**Q: I don't see the Settings menu**
A: You need admin or write access to the repository to change branch settings.

**Q: The 'home' branch doesn't appear in the dropdown**
A: Make sure the branch has been pushed to GitHub first (see Step 1).

**Q: Can I change it back if needed?**
A: Yes! Just follow the same steps and select a different branch.

## What Happens After?

- New repository clones will start on the `home` branch
- Pull requests will default to the `home` branch as base
- The `home` branch will be shown first on the repository page
- GitHub Actions will run on the `home` branch by default

---

For more details about the branch strategy, see:
- [BRANCH_STRATEGY.md](./BRANCH_STRATEGY.md) - Full documentation (English)
- [OHJEET_SUOMEKSI.md](./OHJEET_SUOMEKSI.md) - Lyhyt ohje (Suomeksi)
