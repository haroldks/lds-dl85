import shutil
import subprocess
from multiprocessing import Pool
from functools import partial

from utils import *

DATASET_DIR = "datasets"
CONFIG_BASE_DIR = "optimal_runs_conf"
RESULTS_DIR = "optimal_results"

ALLOW_DISCREPANCIES = [True, False]
DEPTHS = [3, 4]

PARAMETERS = {"support": 1, "use_information_gain": True, "timeout": 600}


# Execution Information

subprocess.run(
    ["cargo", "build", "--release"]
)  # To build a release version in case of modifications


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
    for depth in DEPTHS:
        dir_name = f"{CONFIG_BASE_DIR}/confs_depth_{depth}_allow_dis_{allow_dis}"
        result_dir = f"{RESULTS_DIR}/confs_depth_{depth}_allow_dis_{allow_dis}"
        if os.path.exists(dir_name):
            shutil.rmtree(dir_name)

        os.makedirs(dir_name)
        os.makedirs(result_dir)

        p = dict(PARAMETERS)  # To avoid reference passing

        p["depth"] = depth
        p["allow_discrepancy"] = allow_dis

        parameters.append(p)
        output_dir.append(dir_name)

conf_tuples = list(zip(parameters, output_dir))

for c in conf_tuples:
    generate_config_files(DATASET_DIR, *c, RESULTS_DIR)

conf_files = get_files(CONFIG_BASE_DIR)

with Pool(N_THREADS) as p:
    p.map(partial(run_test, BIN_FILE), conf_files)

# Merge Results

df = get_results_as_csv(RESULTS_DIR, save=f"{RESULTS_DIR}/results.csv")
