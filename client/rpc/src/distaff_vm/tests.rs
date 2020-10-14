/**
 * @file tests.rs
 * @author ming (wmzhou@protonmail.com)
 * @brief distaffvm rpc methors tests.
 * @version 1.00
 * @date 2020-10-03
 *
 * @copyright Copyright@gbctech. All Rights Reserved.
 *
 */
use super::*;
use sp_core::{Bytes};
use std::fs::File;
use std::io::Read;

#[test]
fn should_return_verify_pass() {

	let mut s_program_hash_file = File::open("../../client/rpc/src/distaff_vm/s_program_hash.txt").unwrap();
	let mut s_program_hash = Vec::new();
	s_program_hash_file.read_to_end(&mut s_program_hash).unwrap();

	let mut s_public_input_file = File::open("../../client/rpc/src/distaff_vm/s_public_input.txt").unwrap();
	let mut s_public_input = Vec::new();
	s_public_input_file.read_to_end(&mut s_public_input).unwrap();

	let mut s_outputs_file = File::open("../../client/rpc/src/distaff_vm/s_outputs.txt").unwrap();
	let mut s_outputs = Vec::new();
	s_outputs_file.read_to_end(&mut s_outputs).unwrap();

	let mut s_proof_file = File::open("../../client/rpc/src/distaff_vm/s_proof.txt").unwrap();
	let mut s_proof = Vec::new();
	s_proof_file.read_to_end(&mut s_proof).unwrap();

	let d_program_hash = Bytes(s_program_hash);
	let d_public_input = Bytes(s_public_input);
	let d_outputs = Bytes(s_outputs);
	let d_proof = Bytes(s_proof);

	let runflag;

	let distaff_vm_instance = DistaffVM::new();
	match distaff_vm_instance.distaffvm_verify(d_program_hash, d_public_input, d_outputs, d_proof) {
		Ok(_) => {println!("Execution verified "); runflag = 1},
		Err(msg) => { println!("Failed to verify execution: {}", msg); runflag = 0 ;}
	}
	assert_eq!(runflag, 1, "test distaff verify func fail for comparison");
	println!("distaff vm test ok");
}
