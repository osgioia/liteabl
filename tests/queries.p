/* Complex logic and operators test */
FOR EACH Customer 
    WHERE (Name = "John" OR Name = "Mary") 
    AND Id > 0 
    AND Id <= 100:
    DISPLAY "Result:" Name Id.
END.

/* Search and filtering test */
FIND FIRST Order 
    WHERE Amount >= 150.50 
    AND CustomerId <> 0:
    DISPLAY OrderId Amount.
END.

/* Mixed literals and fields */
FOR EACH Order WHERE Amount GT 100:
    DISPLAY "Order ID:" OrderId "Amount:" Amount.
END.
