import os
import json
import subprocess
import pandas as pd


def generate_config_files(
    datasets_dir, params, out_dir, results_dir
): 
    files = os.listdir(datasets_dir)

    for file in files:
        file_path = os.path.join(datasets_dir, file)
        file_data = params
        file_data["input"] = file_path
        basename = os.path.splitext(os.path.basename(file_path))[0]
        basename = (
            f"tree_{basename}_supp_{params['support']}_depth_"
            f"{params['depth']}_timeout_{params['timeout']}.json"
        )
        file_data["output"] = f"{results_dir}/{out_dir.split('/')[-1]}/{basename}"
        with open(f"{out_dir}/{basename}", "w") as f:
            json.dump(file_data, f)


def get_files(base_dir):
    files_path = list()
    for file in os.listdir(base_dir):
        path = os.path.join(base_dir, file)
        if os.path.isdir(path):
            files_path += get_files(path)
        else:
            files_path.append(path)
    return files_path


def load_json(file_path):
    with open(file_path, "r") as file:
        return json.load(file)


def run_test(bin_file, config):
    print("Run for this conf : ", config)
    subprocess.run([bin_file, "-c", config])
    print("Run over for this conf :", config)


def get_results_as_csv(results_dir, save=None):
    results = get_files(results_dir)
    data = list()
    for fpath in results:
        loaded = load_json(fpath)
        loaded.pop("tree", None)
        data.append(loaded)
    df = pd.DataFrame(data)
    if save is not None:
        df.to_csv(save, index=False)
        print(f"File Saved in {save}")
    return df
