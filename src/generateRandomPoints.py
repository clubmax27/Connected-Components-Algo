import random

def generate_points_file(file_path, num_points, radius):
	if radius <= 0:
		raise ValueError("Radius must be greater than 0.")

	if radius > 1:
		raise ValueError("Radius must be less than or equal to 1.")

	if num_points <= 1:
		raise ValueError("Number of points must be greater than 1.")

	# Generate random points
	points = [(round(random.uniform(0.0, 1.0), 2), round(random.uniform(0.0, 1.0), 2)) for _ in range(num_points)]
	
	# Write to file
	with open(file_path, 'w') as file:
		file.write(f"{radius}\n")
		for point in points:
			file.write(f"{point[0]}, {point[1]}\n")

# Example usage
num_points = 30
radius = 0.2
file_path = 'points.pts'
generate_points_file(file_path, num_points, radius)
print(f"Points file generated at '{file_path}' with {num_points} points.")
