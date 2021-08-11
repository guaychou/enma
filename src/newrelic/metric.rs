#[derive(Debug, Clone, Copy)]
pub enum Metric {
    CpuUsedCore,
    CpuRquestedCore,
    TotalPods,
    ResponseTimeAverage,
    Throughput,
    MemoryHeapUsed,
    ThreadCount,
}

impl Metric {
    pub fn get_query(&self, application_name: &str, start_time: &str, end_time: &str) -> String {
        match self {
            Self::CpuRquestedCore => {
                format!("from Metric SELECT average(k8s.container.cpuRequestedCores) where tags.app = '{}' SINCE {} UNTIL {}", application_name, start_time, end_time)
            }
            Self::CpuUsedCore => {
                format!("from Metric SELECT average(k8s.container.cpuUsedCores) where tags.app = '{}' SINCE {} UNTIL {}", application_name, start_time, end_time)
            }
            Self::MemoryHeapUsed => {
                format!("SELECT average(newrelic.timeslice.value) FROM Metric WHERE appName = '{}' AND metricTimesliceName = 'Memory/Heap/Used' SINCE {} UNTIL {}", application_name, start_time, end_time)
            }
            Self::ResponseTimeAverage => {
                format!("SELECT average(duration) * 1000 FROM Transaction WHERE appName = '{}' AND transactionType = 'Web' SINCE {} UNTIL {}", application_name, start_time, end_time)
            }
            Self::ThreadCount => {
                format!("SELECT average(newrelic.timeslice.value) FROM Metric WHERE appName = '{}' AND metricTimesliceName = 'JmxBuiltIn/Threads/Thread Count' SINCE {} UNTIL {}", application_name, start_time, end_time)
            }
            Self::Throughput => {
                format!("SELECT rate(count(apm.service.transaction.duration), 1 minute) FROM Metric, Transaction WHERE appName = '{}' AND transactionType = 'Web' SINCE {} UNTIL {}", application_name, start_time, end_time)
            }
            Self::TotalPods => {
                format!("FROM K8sContainerSample SELECT uniqueCount(podName) WHERE label.app = '{}' SINCE {} UNTIL {}", application_name, start_time, end_time)
            }
        }
    }
}
