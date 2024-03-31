use plotters::prelude::*;

#[derive(Debug)]
struct Process {
    pid: i32,
    arrival_time: i32,
    burst_time: i32,
}

struct FCFSScheduler {
    processes: Vec<Process>,
}

impl FCFSScheduler {
    fn new() -> Self {
        FCFSScheduler { processes: Vec::new() }
    }

    fn add_process(&mut self, process: Process) {
        self.processes.push(process);
    }

    fn run(&mut self) {
        let mut current_time = 0;
        let mut completed = 0;
        let mut ready_queue: Vec<&Process> = Vec::new();
        let mut gantt_data: Vec<(i32, i32, i32)> = Vec::new();

        while completed != self.processes.len() {
            for process in self.processes.iter() {
                if process.arrival_time <= current_time && !ready_queue.contains(&process) {
                    ready_queue.push(process);
                }
            }

            if let Some(running_process) = ready_queue.first() {
                let start_time = current_time;
                current_time += running_process.burst_time;
                let end_time = current_time;

                gantt_data.push((start_time, end_time, running_process.pid));

                let index = self.processes
                    .iter()
                    .position(|p| p.pid == running_process.pid)
                    .unwrap();

                println!(
                    "Process {} completed: Turnaround Time = {}, Waiting Time = {}",
                    self.processes[index].pid,
                    current_time - self.processes[index].arrival_time,
                    current_time - self.processes[index].arrival_time - self.processes[index].burst_time
                );

                completed += 1;
                ready_queue.remove(0);
            } else {
                current_time += 1;
            }
        }

        // Generate Gantt Chart
        let root = BitMapBackend::new("fcfs_gantt_chart.png", (600, 400)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption("FCFS Gantt Chart", ("sans-serif", 20))
            .margin(20)
            .x_label_area_size(30)
            .y_label_area_size(40)
            .build_cartesian_2d(0..current_time, 0..self.processes.len() as i32)
            .unwrap();

        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(&WHITE.mix(0.3))
            .y_desc("Process")
            .x_desc("Time")
            .axis_desc_style(("sans-serif", 15))
            .draw()
            .unwrap();

        chart.draw_series(
            Rectangle::new(
                gantt_data.iter().map(|(start, end, pid)| {
                    let mut y = *pid - 1;
                    if y < 0 {
                        y = 0;
                    } // Handle potential index issues
                    ((*start, y), (*end - *start, 1))
                }),
                ShapeStyle::from(&BLUE).filled(),
            ),
        ).unwrap()
            .label("Process")
            .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], ShapeStyle::from(&BLUE).filled()));
    }
}
