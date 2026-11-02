use std::io;
use std::thread;

pub fn sum_parallel(limit: u64, workers: usize) -> Result<u128, io::Error> {
    if workers == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "workers must be greater than zero",
        ));
    }

    if limit == 0 {
        return Ok(0);
    }

    let worker_count = workers.min(usize::try_from(limit).unwrap_or(usize::MAX));
    let chunk_size = limit.div_ceil(worker_count as u64);

    thread::scope(|scope| {
        let handles: Vec<_> = (0..worker_count)
            .map(|worker| {
                let start = worker as u64 * chunk_size;
                let end = start.saturating_add(chunk_size).min(limit);
                scope.spawn(move || (start..end).map(u128::from).sum::<u128>())
            })
            .collect();

        handles.into_iter().try_fold(0_u128, |total, handle| {
            let partial_sum = handle.join().map_err(|_| {
                io::Error::other("a worker thread panicked while summing")
            })?;
            Ok(total + partial_sum)
        })
    })
}

#[cfg(test)]
mod tests {
    use super::sum_parallel;

    #[test]
    fn sums_range_across_workers() {
        assert_eq!(sum_parallel(1_000, 4).unwrap(), 499_500);
    }

    #[test]
    fn rejects_zero_workers() {
        assert!(sum_parallel(10, 0).is_err());
    }
}
