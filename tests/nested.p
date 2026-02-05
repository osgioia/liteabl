/* Nesting test */
FOR EACH Customer WHERE Name = "John":
    DISPLAY "Customer:" Name.
    FOR EACH Order WHERE CustomerId = Customer.Id:
        DISPLAY "    Order ID:" OrderId "Amount:" Amount.
    END.
END.
