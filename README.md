# **#WATADB**

Watadb is a database management system developed for the purpose of learning. I want to learn rust and this is the best way to learn in my opinion. Building something and breaking things is the best way to learn. 


Here are the features of this simple DB:

- User should be able to interact with the software via command line
- User should be able to create a new table
- User should be able to enterdata into that table
- Users should be able to query with a simple query language (get all, get where x is y)
- Users should be able to drop table
- Users should be able to edit data in a specific block



Technical information 

- Tables are just folders. The folder name is the name of the table. We can encrypt this in the future. Table also means document.
- Entering data into a table is simply writing to a file and appending ints data.
- Each file has an array of data (DT). This data DT can be of any type, for now, it think it should be a struct
- The document folder should also hold information on the amount of data (array) that is beign held in a data file
-
