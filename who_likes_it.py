from typing import List

def likes(names: List[str]) -> str:
    result = ""
    match len(names):
        case 0:
            result = "no one likes this"
        case 1:
            result = f"{names[0]} likes this"
        case 2:
            result = f"{names[0]} and {names[1]} like this"
        case 3:
            result = f"{names[0]}, {names[1]} and {names[2]} like this"
        case _:
            result = f"{names[0]}, {names[1]} and {len(names) - 2} others like this"
    return result