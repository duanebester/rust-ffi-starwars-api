# Before running, generate the dylib with the following command:
# ```bash
# make build-python
# ```
from starwars import MyCharacter, get_character

async def main():
    character: MyCharacter = await get_character(1)
    print(character.name)

if __name__ == '__main__':
    import asyncio
    asyncio.run(main())
