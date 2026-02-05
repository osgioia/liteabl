-- Drop tables if they exist
DROP TABLE IF EXISTS "Order";
DROP TABLE IF EXISTS Customer;

-- Create Customer table
CREATE TABLE Customer (
    Id INTEGER PRIMARY KEY AUTOINCREMENT,
    Name TEXT
);

INSERT INTO Customer (Name) VALUES ('John');
INSERT INTO Customer (Name) VALUES ('Mary');
INSERT INTO Customer (Name) VALUES ('Peter');

-- Create Order table
CREATE TABLE "Order" (
    OrderId INTEGER PRIMARY KEY AUTOINCREMENT,
    CustomerId INTEGER,
    Amount REAL
);

INSERT INTO "Order" (CustomerId, Amount) VALUES (1, 100.0);
INSERT INTO "Order" (CustomerId, Amount) VALUES (2, 200.0);
INSERT INTO "Order" (CustomerId, Amount) VALUES (1, 150.0);
