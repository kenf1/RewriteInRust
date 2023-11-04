import os
import polars as pl

def main():
    output_folder = f"{os.getcwd()}/Output/"
    df = pl.read_csv(
            f"{output_folder}untidy_output.csv",has_header=False,null_values=[" "] #import csv, no header, replace " " w/ null
        ).drop(
            ["column_1","column_5","column_8","column_10","column_12"]
        ).with_columns(
            pl.col("column_4").fill_null(pl.col("column_6")),
            pl.col("column_7").fill_null(pl.col("column_9")),
            pl.col("column_11").fill_null(pl.col("column_9")).cast(pl.Int64), #fill null -> convert Int64
            pl.col("column_2").str.replace_all(" ","").cast(pl.Int64) #rm " " -> convert Int64
        ).with_columns(
            pl.col("column_7").str.replace_all(" ",""), #rm " "
        ).drop(
            ["column_6","column_9"]
        ).rename(
            {"column_2": "Rank",
            "column_3": "Rank_Change",
            "column_4": "Player_Name",
            "column_7": "Country",
            "column_11": "Total_Points"}
        )
    
    #write to csv
    df.write_csv(f"{output_folder}tidy_output.csv")

if __name__ == "__main__":
    main()
    print("Success")