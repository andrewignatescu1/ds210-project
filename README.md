Goal
 Identify which neighborhoods in New York City have the best and worst air quality based on historical NO₂ and PM2.5 data. Visualize trends across time and rank areas by average pollution levels.

Data swt
  Source: NYC Open Data — Community Air Survey (https://data.cityofnewyork.us)  
  Format: CSV, ~10,000+ rows  
  Columns used: Geo Place Name, Time Period, Data Value


Data Processing

  Used csv::Reader and serde to parse relevant columns: location, time period, pollution value.

Transformations:  
  - Extracted the year from time strings like Winter 2014-15 to 2014
  - Assigned abbreviations like A0, A1 to place names
  - Normalized and stored records in a PollutionRecord struct with:
    - place_abbreviation
    - year
    - pollution_score

Code Structure

Modules

- data.rs: Loads and cleans raw data
- analyze.rs: Computes average pollution and ranks locations
- visualize.rs: Generates matrix heatmaps and bar charts
- main.rs: Coordinates the full workflow

Key Functions & Types

  Structs
  - PollutionRecord: Holds cleaned data for one observation
  - AirQualitySummary: Tracks cleanest and dirtiest places

 Functions
  - load_pollution_data(): Parses CSV and creates abbreviation map
  - analyze_air_quality(): Finds min/max average pollution
  - print_top_clean_and_dirty(): Displays ranked output in terminal
  - draw_matrix_heatmap(): Visualizes (place × year) grid
  - draw_top_clean_and_dirty_bar_chart(): Visualizes top 3 cleanest/dirtiest areas

Main workflow

1. main.rs calls load_pollution_data()  
2. Feeds results into analyze_air_quality()  
3. Prints summaries using print_top_clean_and_dirty()  
4. Calls visualize.rs to render two charts

Tests

Exact test what it print =s out:
andrewignatescu@Andrews-MacBook-Air-2 air_quality_analyzer_rust % cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running unittests src/main.rs (target/debug/deps/air_quality_analyzer_rust-20f12c8f810300c8)

running 2 tests
test tests::test_analyze_air_quality_identifies_extremes ... ok
test tests::test_print_top_clean_and_dirty_runs_without_panic ... ok



test_analyze_air_quality_identifies_extremes
What it checks:
Verifies that analyze_air_quality() correctly computes average pollution for each place and identifies the cleanest and dirtiest areas.
Why it matters:
This function is the core of your analysis logic. A wrong average would invalidate all results. This test ensures it returns correct best/worst place abbreviations based on mock input.
Test logic:
Creates mock data with two places: A1 and A2
A1 has lower average pollution; A2 has higher
Confirms best_place == A1 and worst_place == A2
test_print_top_clean_and_dirty_runs_without_panic
What it checks:
Confirms that the print_top_clean_and_dirty() function runs without crashing or panicking when given valid input.
Why it matters:
This function prints the top 3 cleanest and dirtiest places. While it has no return value, we want to guarantee it handles data safely and doesn’t panic.
Test logic:
Provides mock data for 5 locations with increasing pollution levels
Supplies a dummy map of abbreviations, full names
Verifies that the function executes successfully without throwing any errors

- Visual tests: Reviewed PNG outputs manually for consistency
- Example of terminal output:
Abbreviation Legend:
A75 → Flatlands and Canarsie (CD18)
A28 → Clinton and Chelsea (CD4)
A41 → Jackson Heights (CD3)
A107 → Woodside and Sunnyside (CD2)
A20 → Long Island City - Astoria
A60 → Southwest Queens
A6 → Williamsburg - Bushwick
A99 → Downtown - Heights - Slope
A29 → Ridgewood - Forest Hills
A10 → Bayside - Little Neck
A37 → Throgs Neck and Co-op City (CD10)
A97 → East Harlem (CD11)
A105 → West Queens
A66 → Bensonhurst (CD11)
A9 → Bayside Little Neck-Fresh Meadows
A111 → Brooklyn
A101 → Gramercy Park - Murray Hill
A59 → East New York and Starrett City (CD5)
A33 → South Ozone Park and Howard Beach (CD10)
A77 → Union Square - Lower East Side
A100 → East Flatbush - Flatbush
A82 → Fordham - Bronx Pk
A69 → Morningside Heights and Hamilton Heights (CD9)
A16 → Jamaica
A71 → Kew Gardens and Woodhaven (CD9)
A52 → Long Island City and Astoria (CD1)
A58 → Fordham and University Heights (CD5)
A30 → Willowbrook
A98 → Stapleton - St. George
A39 → Elmhurst and Corona (CD4)
A36 → Jamaica and Hollis (CD12)
A88 → Bedford Stuyvesant - Crown Heights
A95 → Central Harlem - Morningside Heights
A85 → Union Square-Lower Manhattan
A38 → Upper East Side (CD8)
A61 → Ridgewood and Maspeth (CD5)
A67 → Morris Park and Bronxdale (CD11)
A17 → East New York
A94 → Washington Heights
A13 → Coney Island (CD13)
A74 → Flushing - Clearview
A91 → Crotona -Tremont
A79 → High Bridge - Morrisania
A7 → South Beach - Tottenville
A55 → Mott Haven and Melrose (CD1)
A54 → Greenpoint and Williamsburg (CD1)
A113 → New York City
A57 → Midtown (CD5)
A43 → Tottenville and Great Kills (CD3)
A83 → Lower Manhattan
A40 → Bushwick (CD4)
A106 → Northern SI
A96 → Lower East Side and Chinatown (CD3)
A112 → Staten Island
A76 → Upper East Side-Gramercy
A73 → East Harlem
A65 → South Crown Heights and Lefferts Gardens (CD9)
A51 → Hunts Point and Longwood (CD2)
A24 → Hillcrest and Fresh Meadows (CD8)
A103 → Hunts Point - Mott Haven
A110 → Bronx
A53 → Greenwich Village and Soho (CD2)
A80 → Coney Island - Sheepshead Bay
A3 → Flatbush and Midwood (CD14)
A50 → St. George and Stapleton (CD1)
A42 → Bedford Stuyvesant (CD3)
A21 → Sunset Park
A32 → Borough Park (CD12)
A70 → Parkchester and Soundview (CD9)
A72 → Chelsea-Village
A90 → Riverdale and Fieldston (CD8)
A18 → Upper West Side
A31 → Bay Ridge and Dyker Heights (CD10)
A49 → South Beach and Willowbrook (CD2)
A108 → Washington Heights and Inwood (CD12)
A23 → Crown Heights and Prospect Heights (CD8)
A8 → Southeast Queens
A102 → Greenwich Village - SoHo
A93 → Kingsbridge - Riverdale
A0 → Flushing and Whitestone (CD7)
A78 → Bensonhurst - Bay Ridge
A14 → Queens Village (CD13)
A45 → East Flatbush (CD17)
A47 → Financial District (CD1)
A22 → Canarsie - Flatlands
A56 → Brownsville (CD16)
A2 → Rockaway and Broad Channel (CD14)
A84 → Port Richmond
A62 → Northeast Bronx
A12 → Sheepshead Bay (CD15)
A25 → Rego Park and Forest Hills (CD6)
A15 → South Bronx
A26 → Stuyvesant Town and Turtle Bay (CD6)
A27 → Park Slope and Carroll Gardens (CD6)
A11 → Pelham - Throgs Neck
A104 → Queens
A44 → Morrisania and Crotona (CD3)
A35 → Williamsbridge and Baychester (CD12)
A68 → Bayside and Little Neck (CD11)
A5 → Kingsbridge Heights and Bedford (CD7)
A81 → Rockaways
A92 → Greenpoint
A19 → Borough Park
A109 → Manhattan
A63 → Chelsea - Clinton
A34 → Central Harlem (CD10)
A4 → Sunset Park (CD7)
A87 → Southern SI
A86 → Belmont and East Tremont (CD6)
A89 → Upper East Side
A48 → Fort Greene and Brooklyn Heights (CD2)
A1 → Upper West Side (CD7)
A64 → Fresh Meadows
A46 → Highbridge and Concourse (CD4)
Best Air Quality: A43 (Tottenville and Great Kills (CD3))
Worst Air Quality: A79 (High Bridge - Morrisania)

Top 3 Cleanest Areas:
1. Tottenville and Great Kills (CD3) - 12.70 ppb
2. South Beach - Tottenville - 13.00 ppb
3. Rockaway and Broad Channel (CD14) - 13.35 ppb

Top 3 Most Polluted Areas:
1. High Bridge - Morrisania - 38.11 ppb
2. Hunts Point - Mott Haven - 36.74 ppb
3. Crotona -Tremont - 36.19 ppb













Results





- Terminal output shows the correct top 3 cleanest and most polluted areas with full names and scores.


Usage Instructions

Cargo test to run tests first in termina;
Then Cargo run to actual run the program

Output Files
- Heatmap: air_quality_matrix.png ( As seen above)
- Bar chart: top_pollution_chart.png (As seen above)

Runtime
- Instant (< 1 second)
- Works on datasets <20,000 rows without lag







AI-Assistance Disclosure and Other Citations


 CSV Crate Tutorial (docs.rs)
https://docs.rs/csv/latest/csv/tutorial/index.html

 Plotters Crate Documentation (Official graphing library used)
https://docs.rs/plotters/latest/plotters/

 Plotters Book: Basic Data Plotting
https://plotters-rs.github.io/book/basic/basic_data_plotting.html

Read CSV File in Rust | Rust Tutorial -49 | Dr Vipin Classes (YouTube)
https://www.youtube.com/watch?v=QzvStVAu1_w

 Free Video: Read CSV Files with Rust CSV Crate
https://www.youtube.com/watch?v=OTJ7UjHZ2_k

Plotters Heatmap Discussion (Rust Users Forum)
https://users.rust-lang.org/t/plotters-creating-a-spectrogram-heatmap-with-log-scaling/57129
