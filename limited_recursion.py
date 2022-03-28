import itertools
from utils import *
from multiprocessing import Pool

# RUN CONSTANTS

BASE_DIR = "datasets"
BIN_FILE = "target/release/sandbox"
N_THREADS = 8

# DATA FORMATTING CONSTANTS

TO_DROP = [
    "lol",
    "zer",
    "random",
    "dataset_from_tree",
    "test",
    "random_v2",
    "test_v2",
    "paper_test",
    "tic-tac-toe",
    "paper",
    "tic-tac-toe_",
    "tic-tac-toe__",
    "paper_",
]
# LATEX CONSTANTS

LABEL = "tab:limited_recursion"
CAPTION = "Limited Recursion Test"
CLINES = "skip-last;data"


# USED FUNCTIONS


def run_limited_recursion(dataset, export_dir="export_v3"):
    basename = os.path.splitext(os.path.basename(dataset))[0]
    if not os.path.exists(export_dir):
        os.mkdir(export_dir)

    parameters = {
        "support": 1,
        "depth": 3,
        "allow_discrepancy": True,
    }
    result = list()
    for limit in range(1, 5):
        out_dis = f"{export_dir}/{basename}_dis_true_limit_{limit}.json"
        out_dl8 = f"{export_dir}/{basename}_dis_false_limit_{limit}.json"
        subprocess.run(
            [
                BIN_FILE,
                "-i",
                dataset,
                "-o",
                out_dis,
                "-s",
                f'{parameters["support"]}',
                "-d",
                f'{parameters["depth"]}',
                "--use-information-gain",
                "--discrepancy-limit",
                f"{limit}",
                "--allow-discrepancy",
            ]
        )

        e, r, f = load_results(out_dis)

        d = {
            "dataset": basename,
            "features": f,
            "discrepancy": limit,
            "recursion_limit_lds": r,
            "lds_error": e,
        }

        subprocess.run(
            [
                BIN_FILE,
                "-i",
                dataset,
                "-o",
                out_dl8,
                "-s",
                f'{parameters["support"]}',
                "-d",
                f'{parameters["depth"]}',
                "--recursion-limit",
                f"{r}",
                "--use-information-gain",
            ]
        )

        e, r, f = load_results(out_dl8)
        d["dl85_error"] = e
        d["recursion_limit_dl85"] = r

        result.append(d)
    return result


def load_results(pa):
    with open(f"{pa}", "r") as f:
        r = json.load(f)

    error = r["error"]
    recursion_count = r["recursion_count"]
    nb_features = r["nb_features"]
    return error, recursion_count, nb_features


def bolding(line):
    d = "DL8.5"
    ls = "LDS-DL8.5"
    if int(line[("Algorithms Errors", ls)]) < int(line[("Algorithms Errors", d)]):
        line[
            ("Algorithms Errors", ls)
        ] = f'\\textbf{{{line[("Algorithms Errors", ls)]}}}'
        return line
    if int(line[("Algorithms Errors", ls)]) == int(line[("Algorithms Errors", d)]):
        line[("Algorithms Errors", d)] = f'\\textbf{{{line[("Algorithms Errors", d)]}}}'
        line[
            ("Algorithms Errors", ls)
        ] = f'\\textbf{{{line[("Algorithms Errors", ls)]}}}'
        return line
    line[("Algorithms Errors", d)] = f'\\textbf{{{line[("Algorithms Errors", d)]}}}'
    return line


def main():
    files = list()
    for file in os.listdir(BASE_DIR):
        path = os.path.join(BASE_DIR, file)
        files.append(path)

    with Pool(N_THREADS) as p:
        res = p.map(run_limited_recursion, files)

    df = pd.DataFrame(list(itertools.chain.from_iterable(res)))
    df.to_csv("results_limited_recursion.csv", index=False)

    reduced = df[~df.dataset.isin(TO_DROP)]

    # Just the 10 first datasets
    reduced = reduced.astype({"features": int}, errors="raise")
    print(reduced.features.unique())
    reduced = reduced[reduced["features"] >= 119]

    reduced = reduced.sort_values(
        ["features", "dataset", "features"], ascending=[False, False, True]
    )

    reduced.drop(columns=["features"], inplace=True)
    print(reduced.dataset.unique())

    reduced = reduced.set_index(["dataset", "discrepancy"])
    reduced.drop(columns=["recursion_limit_dl85"], inplace=True)
    cols = ["recursion_limit_lds", "dl85_error", "lds_error"]
    reduced = reduced[cols]
    reduced.rename(
        columns={
            "recursion_limit_lds": "Recursion budget",
            "lds_error": "LDS-DL8.5",
            "dl85_error": "DL8.5",
        },
        inplace=True,
    )
    reduced = reduced.astype({"LDS-DL8.5": int, "DL8.5": int}, errors="raise")
    reduced = reduced.astype({"LDS-DL8.5": str, "DL8.5": str}, errors="raise")
    reduced.columns = pd.MultiIndex.from_tuples(
        zip(itertools.repeat("Algorithms Errors"), list(reduced.columns))
    )
    reduced = reduced.apply(lambda x: bolding(x), axis=1)

    styler = reduced.style
    styler.format_index(escape="latex", axis=1).format_index(escape="latex", axis=0)

    latex = styler.to_latex(
        environment="table",
        position_float="centering",
        multirow_align="c",
        multicol_align="c",
        hrules=True,
        sparse_index=True,
        label=LABEL,
        caption=CAPTION,
        clines=CLINES,
    )

    with open("tab.tex", "w") as file:
        file.write(latex)


if __name__ == "__main__":
    main()
