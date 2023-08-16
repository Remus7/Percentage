FROM python

RUN pip install flask requests redis

COPY . /src/

WORKDIR /src/

ENV REDIS_HOST=my-release-redis-master

CMD ["python", "script.py"]
