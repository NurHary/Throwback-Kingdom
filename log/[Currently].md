# Apa yang akan dilakukan

=== To Fix ===
- [ ] UI Agak Aneh
- [x] Mengubah hampir semua resource switch menjadi events seperti di quadtree (distribute / delete)
- [ ] Sistem apabila jumlahnya hanya satu maka tidak usah render nomornya
- [ ] Semua Kebulshitan Quadtree
    - [ ] Masalah insertion disaat distributes
- [ ] Fix tengah dari dua partisi quadtree
    - [x] Modifikasi distribute
    - [x] Modifikasi QT Signals
    - [x] Mengubah dari sistem satu partisi jadi multi partisi (karena sekarang bisa di tengah)
    - [ ] Debugs That Shit

=== Toolings ===
- [x] Tool Sprite Placeholder
- [ ] Activate Tool
- [ ] Menebang Pohon, Drop


=== System Inventory & Building ===
- [x] System untuk draw items di dalamnya
    - [x] Fix untuk draw hanya pada karakter tertentu
- [ ] implementasi prerequisites dan pembangunan serta system grid pembangunan

=== Quadtree ===
- [x] Mengatasi berbagai masalah yang ada dalam quadtree itu sendiri (Almost)
    - [ ] ada masalah ketika memasukkan unit kedalam suatu partisi dengan 4 anakan di dalamnya yang mana
    ketambahan satu lagi akan membuat partisi itu langsung subdivide
    - [ ] ada masalah ketika game di init dan dalam quadtree langsung ada lebih dari 4 karakter, make akan duplikat

=== ERROR / WARN ===
- [ ] SEMUA WARN YANG ADA DI DUNIA INI HARUS DIHILANGKAN
