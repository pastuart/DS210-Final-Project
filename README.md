# DS210-Final-Project

Project Overview:
The goal of this project is to see, from a global scale, the correlation between variables like country size, population, and GDP to the amount of CO2 that that country would produce. This was done through a k-means clustering algorithm to see if there were any trends that we could follow over a roughly 20 year period, both in where the cluster center lies and the attributes of the countries that lie within particular clusters.
Two Datasets were used here and merged together into one large dataset, which is named data.csv within the project. The two original datasets are linked below:
CO2 Emission by countries Year wise (1750-2022)
GDP By Country 1999-2022

Data Processing:
The two data sets were at first loaded into Rust together and unzipped to extract the CSV files.  
The main “CO2 Emission by Countries” dataset was first cleaned up to both remove any units within the actual data values, as well as filling in any empty values and setting them to 0 (or N/A, if it was a word).
Then, GDP by country was added to each of the countries as an extra column of data for the years it was applicable. All countries that didn’t appear in both datasets were removed, as well as all years that weren’t included in both.

Code Structure:
read_csv.rs 
This module was meant to process and manipulate csv files into workable structs that could then later be analyzed graphically by the “cluster.rs” module. Organization-wise, the structs and enums come first, just to give a structure to how exactly the csv files will be packaged into variables, followed by the architecture to actually create a data frame. Some supplementary additions after that include creating subsections of the dataframe which only include a particular country, or all the countries during a particular year.
ColumnVal/DataFrame
These work together in order to represent an CSV file which has columns that are not the same data type. The ColumnVal enum allows for each column to be stored as a different variable type.
Read_csv
Goes through and saves the first line as the labels, and the rest recorded by row into each of their respective columns
cluster.rs
As it currently exists, this only creates a png of a scatter plot given two different categories, represented by labels.
Main workflow
This mainly exists to actually set up the CSVs into workable data, that then can and will be analyzed further with more functionality being added to cluster.rs. 

Usage Instructions
To analyze different parameters, change the x_label and the y_labels in each function to analyze the difference between the two variables in each of the different scenarios
Categories are Country, Year, CO2 Emission(Tons), Population(2022), Area(km^2), Percent of World Population, Population Density(person/km^2), GDP(Billions USD)
No command line prompts necessary, and run time should not be very long
