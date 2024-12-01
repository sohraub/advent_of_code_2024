import os


def part_1_solution(list_a, list_b):
    list_a.sort()
    list_b.sort()

    distances = [abs(a - b) for a, b in zip(list_a, list_b)]
    print("Part 1 answer:", sum(distances))


def part_2_solution(list_a, list_b):
    occurences = {}
    for value in list_b:
        if occurences.get(value, False):
            occurences[value] += 1
        else:
            occurences[value] = 1

    similarity_scores = []
    for value in list_a:
        occurence = occurences.get(value, 0)
        similarity_scores.append(value * occurence)

    print("Part 2 answer:", sum(similarity_scores))



if __name__ == '__main__':

    with open(os.path.join('1', 'puzzle_input.txt'), 'r', encoding='utf-8') as f:
        lines = f.readlines()
        locations_a = []
        locations_b = []
        for line in lines:
            a, b = line.split()
            locations_a.append(int(a))
            locations_b.append(int(b))

    part_1_solution(locations_a, locations_b)
    part_2_solution(locations_a, locations_b)
