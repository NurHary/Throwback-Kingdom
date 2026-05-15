# Changes To How Quadtree Works
## 1. Pendahuluan
Quadtree merupakan algoritma partisi ruang yang digunakan untuk memisahkan ruang / mempersempit query antar unit dalam satu ruang dimana
quadtree menaruh setiap unit berdasarkan posisi / coordinat mereka dalam kotak yang saling membagi diri mereka ketika kotak tersebut
mengandung lebih banyak dari 4 unit yang dipegang

Quadtree ini sangat penting penggunaannya untuk mempercepat dan meringankan operasi yang berkaitan antara dua unit. Quadtree pada project
ini dikhususkan untuk meningkatkan performa dari pengecekan collision antara dua object dengan mempersempit pengecekan ke unit - unit yang ada
di dalam suatu partisi ke antara unit di partisi yang sama, hal tersebut dapat mencegah pengecekan collision dua unit yang tidak perlu seperti
dua unit yang saling terpisah dengan jarak 20km jauhnya

## 2. Dasar Masalah
Sistem dasar dari quadtree ini awalnya melakukan update hanya ketika ada unit yang berada di suautu partisi yang tidak mengandung informasi terkait
unit tersebut pada partisi tersebut. kemudian sistem itu akan langsung melakukan distribusi / migrasi unit tersebut ke partisi baru itu dan
memanggil dua sinyal yang mana:
1.  Melakukan pengecekan apakah partisi yang dimasuki mengalami subdivide dan apabila iya maka semua unit dalam partisi tersebut akan di distribusi
    kan mengikuti posisinya saat ini ke sub partisi
2.  Melakukan penghapusan pada posisi lama dari unit yang melakukan perpindahan

cara berkomunikasi mereka adalah menggunakan sinyal yang terkait pada hal berikut
1.  Distribusi:QTDistributeCondition (Vec3 pos) -> disini dia akan mengubah QuadtreeUnitPosition dari unit tersebut jadi posisi baru (Vec3 pos)
2.  Delete:QTDeleteCondition (Vec3 pos):QuadtreeUnitPosition -> pertama akan dilakukan pengecekan pada qt apakah kedua posisi QuadtreeUnitPosition
    berasal dari dua partisi yang berbeda, apabila iya maka hapus unit itu dari partisi yang lama. lalu minta parent partisi lamanya cek apabila
    lebih tiles lebih kecil dari 4, apabila iya maka remerge

Konsep yang telah disebutkan diatas merupakan konsep yang sangat baik dan sangatlah murah untuk dilakukan karena dengan begitu quadtree hanya
update setiap kali ada yang berpindah dari satu partisi ke partisi lainnya. namun hal tersebut juga membawa kekuarangan fatal:
    tidak ada cara untuk mengassign unit pada quadtree ketika unit tersebut ada di antara dua batas quadtree, sehingga di satu sisi collision
    bekerja sedangkan di satu sisinya tidak dan justru tembus

## 3. Proposed Solution
ada satu solusi yang datang ke pikiran saya terkait cara untuk mengatasi masalah tersebut:
1.  Jadi daripada kita melakukan distribute secara asal - asalan, kita akan memanfaatkan QuadtreeUnitPosition secara sepenuhnya, dimana kita
    akan memastikan sistem dari QuadtreeUnitPosition yang dijalankan pertama. QuadtreeUnitPosition akan mengambil setiap nilai dari kotak collision
    unit yang masuk kedalam suatu partisi akan di list kedalam nilai QuadtreeUnitPosition
2.  lalu lakukan pengecekan apakah QuadtreeUnitPosition yang saat ini masih sama dengan yang beberapa waktu lalu, apabila sama skip, apabila beda
    maka barulah kita distribute unit dan lakukan deletion pada titik yang tidak ada di yang baru

jadi bagaimana apa prerequisite yang dibutuhkan
1.  Remodel Function, disini kita remodel update_quatree_unit dari yang awalnya cek apakah suatu unit ada di partisi yang tidak mencatat
    keberadaannya menjadi fungsi yang **mengcek apabila ada suatu unit yang collisionnya memotong keluar batas suatu partisi** (didekat batas)
    apabila ada maka update QuadtreeUnitPosition. disaat yang bersamaan akan ada state yang memegang variabel bool biner, disaat fungsi ini aktif
    maka state tersebut akan menjadi true semuanya.
2.  QuadtreeUnitPosition melakukan pengecekan nilai barunya dengan nilai lamanya, apabila berbeda panggil signal untuk distribute dan delete
3.  Remodel Fungsi Distribute dan Fungsi Delete:
    1.  fungsi Distribute sekarang terbelah menjadi dua: yang satu untuk insert dengan fungsi distribute dan satunya untuk mengatasi subdivide.
        fungsi pertama akan dipanggil, dan apabila return informasi bahwa terjadi subdivide maka memanggil sinyal untuk handle distribute
    2.  nilai yang dikirimkan pada QTDeleteCondition sekarang bisa menjadi nilai eksak pada titik tersebut
4.  Dan apabila suatu unit tersebut telah pergi dari batas, maka akan mengubah state untuk mentrigger fungsi berikutnya yang mana fungsi itu
    akan melakukan pengecekan QuadtreeUnitPosition terakhir dan pengecekan serta deletion terakhir dimana fungsi ini hanya akan berjalan sekali

Catatan:
    -   pada poin pertama, itu akan berjalan apabila ada transform yang berubah (Change<Transform>)

kunci:
[Component]:
    1. QuadtreeUnitStates {bool st1; bool st2;} -> bool QuadtreeLeavesBorderState; bool QuadtreeUnitOnBorderState


## Masalah Baru
### Sistem QTDEC
Aku Lupa kalau sistem qtdec menggunakan sistem Posisi. masalahnya saat ini adalah posisi update terus selama karakter ada di pojokan
/ menyentuh border, apa solusinya? solusi yang ada di pikiranku adalah membuat sistem id pada quadtree itu sendiri. sebenarnya bisa menggunakan id
dengan nama tapi menurut saya itu sangatlah boros ruang dan mudah untuk tabrakan sehingga solusi yang saya bawakan tentunya adalah dengan menggunakan
tipe data size_t alias usize. dimana ini akan membuat berdasarkan indeks, contohnya:

u64 / usize_t mortonid; menggunakan pendekatan morton id: 00, 01, 10, 11


### Sistem Deletion
ubah qtdec dari menerima QuadtreeIndex menjadi menerima Vec3 dimana kita juga akan melakukan apa yang quadtree awalnya lakukan dimana mereka akan
selalu based on positions
