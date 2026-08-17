# SeaORM + Seaography Example

| ![](https://raw.githubusercontent.com/SeaQL/sea-orm/master/examples/seaography_example/Seaography%20example.png) |
|:--:| 
| Seaography screenshot with Bakery schema |

| ![](https://raw.githubusercontent.com/SeaQL/sea-orm/master/tests/common/bakery_chain/bakery_chain_erd.png) |
|:--:| 
| The Bakery schema |

## Specify a database url

```
export DATABASE_URL=mysql://sea:sea@localhost/bakery
```

## Setup the Database

Cd into `migration` folder and follow instructions there, but basically:

```sh
cargo run
```

## Install Seaography

```sh
cargo install seaography-cli@^1.1.3
```

## Generate GraphQL project

```sh
rm -rf graphql # this entire folder is generated
sea-orm-cli generate entity --output-dir graphql/src/entities --seaography
seaography-cli graphql graphql/src/entities $DATABASE_URL sea-orm-seaography-example
```

## Running the project

```sh
cd graphql
cargo run
```

## Run some queries

### Bakery -> Cake -> Baker

```graphql
{
  bakery(pagination: { page: { limit: 10, page: 0 } }, orderBy: { name: ASC }) {
    nodes {
      name
      cake {
        nodes {
          name
          price
          baker {
            nodes {
              name
            }
          }
        }
      }
    }
  }
}
```

### List gluten-free cakes and know where to buy them

```graphql
{
  cake(filters: { glutenFree: { eq: 1 } }) {
    nodes {
      name
      price
      glutenFree
      bakery {
        name
      }
    }
  }
}
```
