yippie, sekarang:
# Clear Quadtree if no childnode
ada dua: 
1. fungsi dari bevy yang melakukan pengecekan sama seperti distribute itu
    - jajal
2. fungsi pada quadtree tersebut untuk check dan clear apabila anakan kosong
    - for i in childnode
    - if i.divided: recursive
    - 


// jadi ada dua kemungkinan juga untuk terjadi penghapusan partition: ketika entity itu dihapus dan ketika itu berpindah
// jadi: 
// - entity berpindah -> trigger distribute sekalian cek isi partition
// - entity mati -> trigger penghapusan sekalian cek isi partition


untuk bagian raycast saya memiliki ide sebagai berikut:
1. pastikan tr ada
2. tentukan arahnya kemana dengan rumus, kemudian kita ambil ukuran boundaries
