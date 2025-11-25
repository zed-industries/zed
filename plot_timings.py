# import all modules
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import seaborn as sns

# # Read in the DataFrame
# df = pd.read_csv('sync_timings.txt')
# creating a histogram
# plt.hist(df)
# plt.show()

data1 = np.loadtxt("snapshot_timings.txt", delimiter=",")
data2 = np.loadtxt("sync_timings.txt", delimiter=",")

plt.hist(data1, bins = np.arange(0.0, 0.05, 0.00001), label="snapshot")  # Create a histogram with 30 bins
plt.hist(data2, bins = np.arange(0.0, 0.05, 0.00001), label="sync")  # Create a histogram with 30 bins
plt.xlabel("Time between calls")
plt.ylabel("Frequency")
plt.legend()
plt.title("file")
plt.show()
