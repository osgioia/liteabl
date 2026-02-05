/* Full CRUD Lifecycle */

/* 1. Create new customer */
CREATE Customer.

/* 2. Find the most recent customer */
FIND FIRST Customer WHERE Id > 0:
    DISPLAY "New customer created".
END.

/* 3. Cleanup/Delete */
DELETE Customer.

/* 4. Verify empty table */
FOR EACH Customer:
    DISPLAY Name.
END.
