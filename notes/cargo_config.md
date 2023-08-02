## Which Linker are you using.

Use objdump -j .comment to see info from the compile program. Here is my command history. I created the config with the linker configuratoins first. I set it globally.

'''
➜  zero2prod git:(main) vim ~/.cargo/config.toml
➜  zero2prod git:(main) mv ~/.cargo/config.toml ~/.cargo/config.toml.bk
➜  zero2prod git:(main) cargo build
   Compiling zero2prod v0.1.0 (/home/paul/source/rustProjects/zero2prod)
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
➜  zero2prod git:(main) ✗ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/zero2prod`
Hello, world!
➜  zero2prod git:(main) ✗ objdump -j .comment -s target/debug/zero2prod 

target/debug/zero2prod:     file format elf64-x86-64

Contents of section .comment:
 0000 4743433a 2028474e 55292031 322e322e  GCC: (GNU) 12.2.
 0010 31203230 32323131 32312028 52656420  1 20221121 (Red 
 0020 48617420 31322e32 2e312d34 29004743  Hat 12.2.1-4).GC
 0030 433a2028 474e5529 2031322e 332e3120  C: (GNU) 12.3.1 
 0040 32303233 30353038 20285265 64204861  20230508 (Red Ha
 0050 74203132 2e332e31 2d312900           t 12.3.1-1).    
➜  zero2prod git:(main) ✗ mv ~/.cargo/config.toml.bk ~/.cargo/config.toml   
➜  zero2prod git:(main) ✗ cargo build
   Compiling zero2prod v0.1.0 (/home/paul/source/rustProjects/zero2prod)
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
➜  zero2prod git:(main) ✗ objdump -j .comment -s target/debug/zero2prod  

target/debug/zero2prod:     file format elf64-x86-64

Contents of section .comment:
 0000 4743433a 2028474e 55292031 322e322e  GCC: (GNU) 12.2.
 0010 31203230 32323131 32312028 52656420  1 20221121 (Red 
 0020 48617420 31322e32 2e312d34 2900004c  Hat 12.2.1-4)..L
 0030 696e6b65 723a204c 4c442031 352e302e  inker: LLD 15.0.
 0040 37004743 433a2028 474e5529 2031322e  7.GCC: (GNU) 12.
 0050 332e3120 32303233 30353038 20285265  3.1 20230508 (Re
 0060 64204861 74203132 2e332e31 2d312900  d Hat 12.3.1-1).
'''
