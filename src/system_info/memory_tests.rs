use crate::MemInfo;

#[test]
fn parse() {
    let s = "
MemTotal:        6066608 kB
MemFree:          369716 kB
MemAvailable:    1748488 kB
Buffers:          253296 kB
Cached:          1280796 kB
SwapCached:        20500 kB
Active:          1294576 kB
Inactive:        3804432 kB
Active(anon):     230724 kB
Inactive(anon):  3448292 kB
Active(file):    1063852 kB
Inactive(file):   356140 kB
Unevictable:         112 kB
Mlocked:             112 kB
SwapTotal:       7811068 kB
SwapFree:        7359484 kB
Zswap:                 0 kB
Zswapped:              0 kB
Dirty:              1092 kB
Writeback:             0 kB
AnonPages:       3475308 kB
Mapped:           672016 kB
Shmem:            114100 kB
KReclaimable:     255808 kB
Slab:             381652 kB
SReclaimable:     255808 kB
SUnreclaim:       125844 kB
KernelStack:       14896 kB
PageTables:        57380 kB
SecPageTables:         0 kB
NFS_Unstable:          0 kB
Bounce:                0 kB
WritebackTmp:          0 kB
CommitLimit:    10844372 kB
Committed_AS:   11615308 kB
VmallocTotal:   34359738367 kB
VmallocUsed:       43596 kB
VmallocChunk:          0 kB
Percpu:             2784 kB
HardwareCorrupted:     0 kB
AnonHugePages:    718848 kB
ShmemHugePages:        0 kB
ShmemPmdMapped:        0 kB
FileHugePages:         0 kB
FilePmdMapped:         0 kB
HugePages_Total:       0
HugePages_Free:        0
HugePages_Rsvd:        0
HugePages_Surp:        0
Hugepagesize:       2048 kB
Hugetlb:               0 kB
DirectMap4k:      258944 kB
DirectMap2M:     4982784 kB
DirectMap1G:     3145728 kB
";
    let mem_info: MemInfo = s.parse().unwrap();

    let mem_info_orig = MemInfo {
        mem_total: 6066608,
        mem_free: 369716,
        mem_available: 1748488,
        swap_total: 7811068,
        swap_free: 7359484,
    };

    assert_eq!(mem_info, mem_info_orig);
}
