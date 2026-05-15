# 08-05-2026
Error besar terjadi pada quadtree dimana saya ingin menambahkan sistem untuk quadtree dapat bekerja ditengah. ada beberapa hal yang saya temui dari
error tersebut
1.  Sistem Penambahan entity pada quadtree ini hanya akan update ketika suatu entity berpindah dari satu partisi ke partisi lainnya.
    jadi ketika saya berusaha menambahkan sistem untuk berada ditengah yang mana ini saya menambahkan posisi satu unit di satu partisi
    ketika suatu unit pindah yang terjadi adalah penghapusan tapi yang saat bersamaan menambahkan unit pada quadtree itu karena collisionnya
    masih masuk di quadtree tersebut

solusi:
1.  pada update_quadtree_unit daripada melakukan pengecekan unit ketika ada yang masuk ke area baru. kita akan melakukan pengecekan
    apabila suatu unit rectanglenya intersect dengan batas dari suatu border dimana akan dilakukan update terus menerus dan ketika entity tersebut
    telah keluar dari area border tersebut, maka dilakukan aktivasi delete

# 27-02-2026
kebanyakan yang dilakukan saat ini masih revisi serta persiapan untuk tkinventory, untuk saat ini tkinvetory dipersiapkan sampai
fungsi check_contains_items saja dengan tujuan untuk memberikan index. sebelumnya itu sendiri saya telah mendesain bagaimana system
pengambilan akan bekerja mulai dari **check_contains_items yang apabila ada make tambah pada index dan apabila tidak ada append di vec;
begitu juga ketika penambahan di bagian sisa negatif (artinya items di inv dan yang mau diambil lebih besar daripada max stack) make
tambahkan lalu split data, hasil dari split kemudian di append ke dalam inventory tersebut;**
