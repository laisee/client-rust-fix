# Fix Acceptor Pattern

### Overview

In a FIX (Financial Information Exchange) trading application, the acceptor pattern is essential for handling incoming connections and messages from FIX initiators (clients). 

The acceptor pattern is responsible for establishing and managing the connections, authenticating clients, and processing the received messages according to the FIX protocol. 

Here’s a detailed description of an acceptor pattern for a FIX trading app:

### 1. Connection Management

Listener Setup: The acceptor sets up a listener on a specified port to accept incoming FIX connections.
Connection Handling: When a new connection request is received, the acceptor creates a new session for each connection. This involves allocating resources and establishing the necessary data structures to manage the session.

### 2. Session Management
Session Identification: Each session is uniquely identified by a combination of the SenderCompID and TargetCompID (unique identifiers for the sender and receiver).
Session Storage: The acceptor maintains a session storage mechanism to keep track of active sessions and their states.

### 3. Message Reception and Parsing
Message Buffering: Incoming messages are received into a buffer. The acceptor ensures that messages are fully received before processing.
Message Parsing: The acceptor parses the received FIX messages according to the FIX protocol specifications. This involves breaking down the raw message into its constituent tags and values.

### 4. Authentication and Authorization
Client Authentication: The acceptor authenticates the incoming connection based on pre-configured credentials or certificates.
Authorization: After authentication, the acceptor authorizes the client to ensure they have the necessary permissions to perform the requested actions.

### 5. Business Logic Handling
Message Routing: Parsed messages are routed to the appropriate business logic handler based on the message type (e.g., New Order, Order Cancel, Execution Report).
Processing Logic: The business logic handler processes the message, which may involve validating the message, executing trades, updating order books, etc.

### 6. Response Generation
Acknowledge Receipt: The acceptor generates acknowledgment messages (e.g., Logon, Heartbeat, Test Request) as specified by the FIX protocol.
Execution Reports: For trade-related messages, the acceptor generates execution reports and sends them back to the initiator.

### 7. Error Handling
Protocol Errors: The acceptor handles protocol errors by sending appropriate reject messages.
Connection Errors: The acceptor manages connection errors, such as disconnections or timeouts, and takes appropriate action, such as attempting reconnections or closing sessions.

### 8. Logging and Monitoring
Message Logging: The acceptor logs all incoming and outgoing messages for audit and troubleshooting purposes.
Session Monitoring: The acceptor monitors the health and status of all active sessions, ensuring they are functioning correctly.

### Acceptor Sequence Diagram

<img width="520" alt="Screenshot 2024-06-17 at 10 18 47" src="https://github.com/laisee/client-rust-fix/assets/5905130/03187df4-353a-49ad-b0ce-5a1c98368fda">


## Sample code in Python

Here's a simplified example using the QuickFIX library in Python, which is a popular FIX engine:

    import quickfix as fix

    class Application(fix.Application):
        def onCreate(self, sessionID):
            print(f"Session created: {sessionID}")

        def onLogon(self, sessionID):
            print(f"Logon: {sessionID}")

        def onLogout(self, sessionID):
            print(f"Logout: {sessionID}")

        def toAdmin(self, message, sessionID):
            print(f"To Admin: {message}")

        def fromAdmin(self, message, sessionID):
            print(f"From Admin: {message}")

        def toApp(self, message, sessionID):
            print(f"To App: {message}")

        def fromApp(self, message, sessionID):
            print(f"From App: {message}")
        self.processMessage(message, sessionID)

    def processMessage(self, message, sessionID):
        # Implement business logic based on message type
        msg_type = message.getHeader().getField(fix.MsgType().getValue())
        if msg_type == fix.MsgType_NewOrderSingle:
            self.processNewOrder(message, sessionID)

    def processNewOrder(self, message, sessionID):
        # Handle New Order Single message
        print("Processing new order")

    # Configuration file path
    config_file = "acceptor.cfg"

    # Initialize and start the acceptor
    settings = fix.SessionSettings(config_file)
    application = Application()
    store_factory = fix.FileStoreFactory(settings)
    log_factory = fix.FileLogFactory(settings)
    acceptor = fix.SocketAcceptor(application, store_factory, settings, log_factory)
    acceptor.start()

    # Run the application
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        acceptor.stop()
