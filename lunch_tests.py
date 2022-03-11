import shutil
from multiprocessing import Pool
from functools import partial

from utils import *

# Directories constants
DATASET_DIR = "datasets"
CONFIG_BASE_DIR = "confs_dir"
RESULTS_DIR = "CONFS_RESULTS"

# TEST PARAMETERS
ALLOW_DISCREPANCIES = [True]
USE_INFORMATION_GAIN = [True]
TIMEOUTS = [10]
PARAMETERS = {
    "support": 100,
    "depth": 3,
}

# Execution Information
BIN_FILE = "target/release/sandbox"
N_THREADS = 8

if os.path.exists(RESULTS_DIR):
    shutil.rmtree(RESULTS_DIR)

if os.path.exists(CONFIG_BASE_DIR):
    shutil.rmtree(CONFIG_BASE_DIR)


os.makedirs(RESULTS_DIR)
os.makedirs(CONFIG_BASE_DIR)

parameters = list()
output_dir = list()

for allow_dis in ALLOW_DISCREPANCIES:
    for use_ig in USE_INFORMATION_GAIN:
        for time in TIMEOUTS:
            dir_name = f"{CONFIG_BASE_DIR}/confs_time_{time}_allow_dis_{allow_dis}_use_ig_{use_ig}"
            result_dir = f"{RESULTS_DIR}/confs_time_{time}_allow_dis_{allow_dis}_use_ig_{use_ig}"
            if os.path.exists(dir_name):
                shutil.rmtree(dir_name)

            os.makedirs(dir_name)
            os.makedirs(result_dir)

            p = dict(PARAMETERS)  # To avoid reference passing
            p["allow_discrepancy"] = allow_dis
            p["use_information_gain"] = use_ig
            p["timeout"] = time
            parameters.append(p)
            output_dir.append(dir_name)

conf_tuples = list(zip(parameters, output_dir))

for c in conf_tuples:
    generate_config_files(DATASET_DIR, *c, RESULTS_DIR)

# Run Tests
conf_files = get_files(CONFIG_BASE_DIR)

with Pool(N_THREADS) as p:
    p.map(partial(run_test, BIN_FILE), conf_files)

# Merge Results

df = get_results_as_csv(RESULTS_DIR, save=f"{RESULTS_DIR}/results.csv")

# # Remove Outputs
