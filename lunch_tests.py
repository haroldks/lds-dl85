import os
import shutil
import json
import subprocess
from multiprocessing import Pool

# Directories constants
DATASET_DIR = "datasets"
CONFIG_BASE_DIR = "confs_dir"
RESULTS_DIR = "CONFS_RESULTS"


# TEST PARAMETERS
ALLOW_DISCREPANCIES = [True, False]
USE_INFORMATION_GAIN = [True, False]
TIMEOUTS = [30, 60, 90]
PARAMETERS = {
    "support": 1,
    "depth": 9,
}

# Execution Information
BIN_FILE = "target/release/sandbox"
N_THREADS = 8


if os.path.exists(RESULTS_DIR):
    shutil.rmtree(RESULTS_DIR)
os.makedirs(RESULTS_DIR)


def generate_config_files(datasets_dir, params, out_dir):  # TODO: Allows Multiple timeout ?
    files = os.listdir(datasets_dir)

    for file in files:
        file_path = os.path.join(datasets_dir, file)
        file_data = params
        file_data['input'] = file_path
        basename = os.path.splitext(os.path.basename(file_path))[0]
        basename = f"tree_{basename}_supp_{params['support']}_depth_" \
                   f"{params['depth']}_timeout_{params['timeout']}.json"
        file_data["output"] = f"{RESULTS_DIR}/{out_dir.split('/')[-1]}/{basename}"
        with open(f"{out_dir}/{basename}", "w") as f:
            json.dump(file_data, f)


def get_conf_files(base_dir):
    files_path = list()
    for file in os.listdir(base_dir):
        path = os.path.join(base_dir, file)
        if os.path.isdir(path):
            files_path += get_conf_files(path)
        else:
            files_path.append(path)
    return files_path


def run_test(config):
    subprocess.run([BIN_FILE, '-c', config])


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
    generate_config_files(DATASET_DIR, *c)

# Run Tests

conf_files = get_conf_files(CONFIG_BASE_DIR)

with Pool(N_THREADS) as p:
    p.map(run_test, conf_files)

#
#
# def launch_rust(config):
#     subprocess.run(['./target/release/sandbox', '-c', config])
#
#
# # Merge Results
#
#
# # Remove Outputs
