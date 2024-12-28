# Constructors

- With the release and successful validator vote of Protocol 22, smart contracts are now capable of utilizing a __constructor function!

- it's possible to perform that initialization at deploy-time. This prevents front-running, and keeps the contract you've deployed within your own control at all times.

- Constructor functions look pretty much exactly the same as the previously used init functions. The only difference is when the function is executed.

- 


[ConstructorContract](https://github.com/ElliotFriend/ye-olde-guestbook/blob/main/contracts/ye_olde_guestbook/src/lib.rs)
[ConstructorTest](https://github.com/ElliotFriend/ye-olde-guestbook/blob/main/contracts/ye_olde_guestbook/src/test.rs)