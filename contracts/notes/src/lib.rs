#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data Mahasiswa
#[contracttype]
#[derive(Clone, Debug)]
pub struct Mahasiswa {
    id: u64,
    nim: String,
    nama: String,
    jurusan: String,
}

// Storage key
const MHS_DATA: Symbol = symbol_short!("MHS_DATA");

#[contract]
pub struct MahasiswaContract;

#[contractimpl]
impl MahasiswaContract {

    // Ambil semua data mahasiswa
    pub fn get_mahasiswa(env: Env) -> Vec<Mahasiswa> {
        env.storage().instance().get(&MHS_DATA).unwrap_or(Vec::new(&env))
    }

    // Tambah mahasiswa
    pub fn create_mahasiswa(env: Env, nim: String, nama: String, jurusan: String) -> String {
        let mut data: Vec<Mahasiswa> = env.storage().instance().get(&MHS_DATA).unwrap_or(Vec::new(&env));

        let mhs = Mahasiswa {
            id: env.prng().gen::<u64>(),
            nim: nim,
            nama: nama,
            jurusan: jurusan,
        };

        data.push_back(mhs);
        env.storage().instance().set(&MHS_DATA, &data);

        String::from_str(&env, "Mahasiswa berhasil ditambahkan")
    }

    // Hapus mahasiswa berdasarkan id
    pub fn delete_mahasiswa(env: Env, id: u64) -> String {
        let mut data: Vec<Mahasiswa> = env.storage().instance().get(&MHS_DATA).unwrap_or(Vec::new(&env));

        for i in 0..data.len() {
            if data.get(i).unwrap().id == id {
                data.remove(i);
                env.storage().instance().set(&MHS_DATA, &data);
                return String::from_str(&env, "Mahasiswa berhasil dihapus");
            }
        }

        String::from_str(&env, "Mahasiswa tidak ditemukan")
    }
}