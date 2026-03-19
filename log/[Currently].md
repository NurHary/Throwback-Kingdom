# Apa yang akan dilakukan

=== To Fix ===
- [x] Masih ingat dengan tipe data doubly linked list yang saya buat, ayo kita ubah dari doubly linked list menjadi linked list bias
    untuk menyimpan lebih banyak memori
- [ ] Mengubah hampir semua resource switch menjadi events seperti di quadtree (distribute / delete)
- [ ] Mencari tahu cara despawn yang baik di bevy serta bagaimana caranya menghapus suatu entities yang di despawn untuk menghilang dari quadtree


=== System Pickup Items ===
- [x] membuat kode / implementasi simpel untuk mengakses quadtree serta menggunakannya sebagai sarana utama untuk melakukan pengecekan
    collision pada semua EntityColliding
- [x] system untuk mengecek collision berdasarkan hasil quadtree
- [x] implementasi simpel untuk EntityColliding
- [x] belajar bevy events - observers serta bevy msg
- [x] impelementasi pickup - Kurang bagian masukkan dalam inventorynya

=== System Inventory & Building ===
- [x] membuat indikator untuk mengecek inventory secara sederhana dengan menggunakan bevy native ui
    - [x] System untuk mendespawn inventory tersebut ketika ganti karakter
- [x] Implementasi pickup ke dalam inventory
- [ ] System untuk draw items di dalamnya
- [ ] implementasi prerequisites dan pembangunan serta system grid pembangunan

=== Quadtree ===
- [ ] Mengatasi berbagai masalah yang ada dalam quadtree itu sendiri (Almost)
    - [ ] ada masalah ketika memasukkan unit kedalam suatu partisi dengan 4 anakan di dalamnya yang mana
    ketambahan satu lagi akan membuat partisi itu langsung subdivide
    - [ ] ada masalah ketika game di init dan dalam quadtree langsung ada lebih dari 4 karakter, make akan duplikat
