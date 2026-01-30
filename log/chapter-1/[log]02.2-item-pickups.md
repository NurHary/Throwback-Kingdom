# [Desc]: Log Berisikan Bagaimana system pickup item dan drop items bekerja

## Masalah
### System deteksi collision dilakukan di tkphysics
ini menjadi masalah utama dari pengembangan system pickup items itu sendiri dimana niatnya saya ingin melakukan system pickup di implementasi
di tkinventory

#### solusi sementara
1.  solusi sementara untuk hal itu pertama adalah dengan membuat system resource switch dimana ketika resource switch itu aktif make akan melakukan
    aksi untuk insert item secara terpisah
2.  Solusi sementara untuk hal itu juga adalah memisahkan sistemnya dari tkphysics dimana tkphysics hanya untuk collision dengan unit sedangkan
    untuk item semuanya murni dilakukan dalam plugins inventory
