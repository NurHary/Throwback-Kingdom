# Physics Systems Untuk Throwback Kingdoms
untuk files physics system itu sendiri memiliki beberapa deklarasi physics seperti bentuk
segi empat dan bentuk kapsul (tidur). selainn itu juga file tersebut juga memiliki beberapa
fungsi yang dapat digunakan oleh main games, fungsi itu terdiri dari hal berikut:
1. fungsi untuk mengakses Quadtree dan mendapatkan entities berdsarkan tingkatan partisi
2. pengecekan collision untuk setiap entities yang dikirim oleh proses 1
3. pengaplikasian physics berdasarkan tipe dari collision tersebut

## Component & Plugins
ada beberapa Component yang di definisikan, yaitu:
1. TkRect -> berguna untuk memberikan bentuk segi empat / rectangle,
2. TkCapsules -> berguna untuk memberikan bentuk kapsul, dan
3. EntityColliding -> menyimpan informasi tipe collision (Enum) serta status collision (bool)

dimana juga terdapat beberapa tipe data yang dijelaskan serta struct plugins
1. CollisionType -> tipe data enum yang memberikan opsi terkait tipe collision
2. TkPhysicsPlugins -> plugins yang menjalankan operasi fungsi dari TkPhysics secara keseluruhan
    segala operasi tersebut (untuk saat ini adalah)
    1. operasi pengecekan collision
    2. #ToImplement operasi pengaksesan quadtree -> Update

