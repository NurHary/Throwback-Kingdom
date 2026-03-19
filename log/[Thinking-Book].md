## Masalah saat ini:
### system Inventory itu sendiri
jadi saya melakukan hal berikut untuk melakukan penambahan pada inventories

Check Inventory <option(usiez, u8)>
1. Pertama mendapatkan item dari observer (A)
2. Mendapatkan query dari entity dan inventory (B) yang melakukan tabrakan
3. Cek Apakah Inventory (B) mengandung items (A) [check_items]
    Check Items <option(u8)>
    1. check self.id dengan rhs.id, apabila tidak return none
    2. apabila iya, make cek apakah self.amount + rhs.amount > MAX
    3. apabila iya, make return MAX - total.amount. dan apabila tidak return 0
4.


45, 25
A = 45 -> 25
B = 45 - 25 = 20
A + B = 45
