📂 Project Structure (Original)
contracts/
└── notes/
    ├── src/
    │   ├── lib.rs      # Main smart contract logic
    │   └── test.rs     # Testing file
    ├── Cargo.toml      # Rust dependencies (contract)
    ├── Cargo.lock      # Dependency lock file
    ├── Makefile        # Build & deploy helper
    ├── Cargo.toml      # Workspace configuration
    └── README.md       # Project documentation

📌 Note:
Struktur ini tetap digunakan tanpa perubahan, sesuai dengan standar project Soroban.

⚙️ Smart Contract Explanation

The smart contract defines a data structure called Mahasiswa (Student):

pub struct Mahasiswa {
    id: u64,
    nim: String,
    nama: String,
    jurusan: String,
}

Each student has:

id → unique identifier (auto-generated)
nim → student number
nama → student name
jurusan → major
🚀 Main Functions
🔹 create_mahasiswa()

Adds a new student to blockchain storage.

Generates unique ID
Stores data permanently
Returns success message
🔹 get_mahasiswa()

Retrieves all student data.

Returns a list of students
Reads directly from blockchain storage
🔹 delete_mahasiswa()

Deletes a student based on ID.

Searches data using loop
Removes matching record
Updates storage
🔗 Deployment Details
Network: Stellar Testnet
Platform: StellarExpert
Contract Address:
CBLU4IUASQ4WUMOXBFLZRSBBLILGOH33GS4LUPKFBCCCMJCDQNMF7G2M
📸 Proof of Deployment

The smart contract has been successfully deployed on the Stellar Testnet.

Evidence includes:

Transaction status: ✅ Successful
Contract created from WASM
Verified ledger entry
Digital signature validation

(See attached screenshot in project)

🔄 System Workflow
User calls create_mahasiswa()
Data is stored on blockchain
User retrieves data using get_mahasiswa()
User deletes data using delete_mahasiswa()
Blockchain ensures all operations are secure and immutable
🧪 Testing

Testing is implemented in:

src/test.rs

Test scenarios:

Add student
Retrieve list
Delete student
Handle invalid ID
🛠️ Technologies Us
Rust Programming Language
Soroban SDK
Stellar Blockchain
WebAssembly (WASM)
🔮 Future Improvements
✏️ Update student data
🔍 Search by NIM
✅ Input validation
🌐 Frontend integration (React)
🔐 Authentication system
🧠 Conclusion

This project demonstrates how blockchain technology can be applied to student data management systems. By using Soroban smart contracts on Stellar, the system ensures security, transparency, and decentralization, making it a reliable alternative to traditional systems.
