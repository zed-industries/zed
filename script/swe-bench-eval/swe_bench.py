# %%
import polars as pl

df = pl.read_parquet('hf://datasets/princeton-nlp/SWE-bench_Verified/data/test-00000-of-00001.parquet')

print(df.head())
print(df.columns)
print(len(df))

# Inspect the head of specific columns
df.select(['repo', 'problem_statement', 'test_patch', 'hints_text']).head()
full_row = df.head(1).to_dict(as_series=False)
import pprint

pp = pprint.PrettyPrinter(indent=4)

print("Repo:")
pp.pprint(full_row['repo'])
print("\nPatch:")
pp.pprint(full_row['patch'])
print("\nTest Patch:")
pp.pprint(full_row['test_patch'])
print("\nProblem Statement:")
pp.pprint(full_row['problem_statement'])
print("\nHints Text:")
pp.pprint(full_row['hints_text'])
print("\nPASS_TO_PASS:")
pp.pprint(full_row['PASS_TO_PASS'])
print("\nFAIL_TO_PASS:")
pp.pprint(full_row['FAIL_TO_PASS'])
