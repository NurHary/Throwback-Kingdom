# Inventory System dalam Throwback Kingdoms
jadi untuk sistem inventory, saya berniat untuk **menggunakan satu Komponen sebagai tempat penyimpanan
dan satu komponen lagi untuk memberikan indikasi jarak akses dari inventory tersebut (bisa antara
diganti dengan Enum atau murni itu saja, menurutku pakai bentuk komponen saja karena bisa menghemat
memori karena yang dilakukan proses hanyalah yang memilikinya)**

dengan kata lain, akan ada 2 struct component:
1. TkInventory -> Lebih gampang dibuat untuk saat ini (megnecualikan insert)
2. TkSharedInventory -> Lebih susah karena memiliki area collision
