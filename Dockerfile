FROM python
COPY . .
RUN pip install -r requirements/dev.txt
ENTRYPOINT [ "python3", "-m", "src.claim_withdrawal"]
