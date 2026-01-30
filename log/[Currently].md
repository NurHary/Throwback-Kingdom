# Apa yang akan dilakukan

=== To Fix ===
- [x] Fugsi pengecekan bekerja pada satu saja
- [x] Pengecekan pada quadtree multi partisi agak awur / tidak memiliki nilai - nilai tertentu
- [ ] masih ingat dengan tipe data doubly linked list yang saya buat, ayo kita ubah dari doubly linked list menjadi linked list biasa
    untuk menyimpan lebih banyak memori


=== System Pickup Items ===
- [x] membuat kode / implementasi simpel untuk mengakses quadtree serta menggunakannya sebagai sarana utama untuk melakukan pengecekan
    collision pada semua EntityColliding
- [x] system untuk mengecek collision berdasarkan hasil quadtree
- [x] implementasi simpel untuk EntityColliding
- [ ] belajar bevy events
- [ ] impelementasi pickup

=== System Inventory & Building ===
- [ ] membuat indikator untuk mengecek inventory secara sederhana dengan menggunakan bevy native ui
- [ ] implementasi prerequisites dan pembangunan serta system grid pembangunan

=== Quadtree ===
- [ ] Mengatasi berbagai masalah yang ada dalam quadtree itu sendiri (Almost)
    - [ ] ada masalah ketika memasukkan unit kedalam suatu partisi dengan 4 anakan di dalamnya yang mana
    ketambahan satu lagi akan membuat partisi itu subdivide
